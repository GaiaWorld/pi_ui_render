/// 字体接口定义

use std::{
	hash::Hash, 
	collections::hash_map::Entry, 
	sync::atomic::{AtomicUsize, Ordering}
};

use ordered_float::NotNan;
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_flex_layout::prelude::Size;
use pi_hash::XHashMap;
use pi_render::rhi::{asset::RenderRes, device::RenderDevice, RenderQueue};
use pi_share::Share;
use pi_slotmap::{DefaultKey, SlotMap};
use wgpu::{TextureView, Texture};

use pi_atom::Atom;

use crate::{components::user::FontSize, utils::tools::calc_hash};

use super::{text_pack::TextPacker, brush_freetype::FreeTypeBrush};


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Font {
	pub font_family: Atom,
	pub font_size: usize,
	pub font_weight: usize,
	pub stroke: NotNan<f32>,
}

impl Font {
	pub fn new(font_family: Atom, font_size: usize, font_weight: usize, stroke: NotNan<f32>) -> Self {
		Self {
			font_family,
			font_size,
			font_weight,
			stroke,
		}
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut)]
pub struct GlyphId(pub DefaultKey);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut)]
pub struct FontId(DefaultKey);

pub trait FontSheet {
	/// 字体id
	fn font_id(&mut self, f: Font) -> FontId;

	/// 字体高度
	fn font_height(&self, f: FontId, font_size: usize) -> f32;

	/// 取到字形id， 如果字体Id（FontId）不存在，将panic
	fn glyph_id(&mut self, f: FontId, char: char) -> Option<GlyphId>;

	/// 测量宽度
	fn measure_width(&mut self, f: FontId, char: char) -> f32;

	/// 取到字形信息
	fn glyph(&self, id: GlyphId) -> &Glyph;

	/// 绘制文字
	fn draw(&mut self);

	/// 取到纹理
	fn texture_view(&self) -> &Handle<RenderRes<TextureView>>;

	/// 取到纹理版本
	fn texture_version(&self) -> usize;

	/// 取到纹理版本
	fn texture_size(&self) -> Size<usize>;

	/// 清理字形信息
	fn clear(&mut self);
}

pub trait FontBrush {
	/// 测量文字尺寸
	fn width(&mut self, font_id: FontId, font: &Font, char: char) -> f32;
	/// 取字体高度
	fn height(&mut self, font_id: FontId, font: &Font) -> f32;
	/// 绘制, 需在texture上正确的位置填充文字像素，完成后，应该更新texture_version
	fn draw(&mut self, sheet: &GlyphSheet);

}

pub struct FontMgr {
	sheet: GlyphSheet,
	brush: FreeTypeBrush,
}

impl std::ops::Deref for FontMgr {
    type Target = GlyphSheet;

    fn deref(&self) -> &Self::Target {
        &self.sheet
    }
}

impl std::ops::DerefMut for FontMgr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sheet
    }
}

pub struct GlyphSheet {
	fonts_map: XHashMap<Font, FontId>,
	fonts: SlotMap<DefaultKey, FontInfo>,
	glyph_id_map: XHashMap<(FontId, char), GlyphId>,
	glyphs: SlotMap<DefaultKey, GlyphIdDesc>,

	text_packer: TextPacker,

	/// 文字纹理（所有用到的文字，都必须包含在该纹理内）
	texture_view: Handle<RenderRes<TextureView>>,
	texture: Share<Texture>,
	texture_width: AtomicUsize,
	texture_height: AtomicUsize,
	texture_version: AtomicUsize,
}

impl GlyphSheet {
	pub fn fonts(&self) -> &SlotMap<DefaultKey, FontInfo> {
		&self.fonts
	}

	pub fn glyphs(&self) -> &SlotMap<DefaultKey, GlyphIdDesc> {
		&self.glyphs
	}

	pub fn update_version(&self) {
		self.texture_version.fetch_add(1, Ordering::Relaxed);
	}

	pub fn update_width(&self, r: usize) {
		self.texture_width.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
			if r > value  {
				return Some(r);
			} else {
				None
			}
		}).unwrap();
	}

	pub fn update_height(&self, r: usize) {
		self.texture_height.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
			if r > value  {
				return Some(r);
			} else {
				None
			}
		}).unwrap();
	}

	pub fn texture(&self) -> &Share<Texture> {
		&self.texture
	}

	pub fn texture_size(&self) -> Size<usize> {
		Size { 
			width: self.texture_width.load(Ordering::Relaxed), 
			height: self.texture_height.load(Ordering::Relaxed) }
	}
}

impl FontMgr {
	pub fn new(
		device: &RenderDevice,
		texture_asset_mgr: &Share<AssetMgr<RenderRes<TextureView>>>,
		queue: &RenderQueue,
	) -> FontMgr {
		let size = 1024;
		let init_height = 1024;
		let brush = FreeTypeBrush::new(queue.clone());
		let texture = (**device).create_texture(&wgpu::TextureDescriptor {
			label: Some("first depth buffer"),
			size: wgpu::Extent3d {
				width: size,
				height: init_height,
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
		});
		let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let key = calc_hash(&"text texture view");
		texture_asset_mgr.cache(key, RenderRes::new(texture_view, 1024 * 1024 * 4));
		let texture_view = texture_asset_mgr.get(&key).unwrap();

		Self { 
			sheet: GlyphSheet {
				fonts_map: XHashMap::default(), 
				fonts: SlotMap::default(), 
				glyph_id_map: XHashMap::default(), 
				glyphs: SlotMap::default(), 
				text_packer: TextPacker::new(size as usize, size as usize), 
				texture_view: texture_view, 
				texture: Share::new(texture), 
				texture_width: AtomicUsize::new(size as usize), 
				texture_height: AtomicUsize::new(init_height as usize), 
				texture_version: AtomicUsize::new(0), 
			},
			brush
		}
	}
}

impl FontSheet for FontMgr {
	/// 字体id
	fn font_id(&mut self, f: Font) -> FontId {
		match self.sheet.fonts_map.entry(f.clone()) {
			Entry::Occupied(r) => r.get().clone(),
			Entry::Vacant(r) => {
				let id = self.sheet.fonts.insert(FontInfo {
					font: f,
					height: 0.0,
					await_info: AwaitInfo { 
						size: Size {width: 0, height: 0}, 
						wait_list: Vec::new() },
				});
				let id = r.insert(FontId(id)).clone();
				let height = self.brush.height(id, &self.sheet.fonts[*id].font);
				self.sheet.fonts[*id].height = height;
				id
			}
		}
	}

	fn font_height(&self, f: FontId, font_size: usize) -> f32 {
		match self.sheet.fonts.get(*f) {
			Some(r) => r.height * (font_size as f32 / BASE_FONT_SIZE as f32),
			None => font_size as f32, // 异常情况，默认返回font_size
		}
	}

	/// 字形id, 纹理中没有更多空间容纳时，返回None
	fn glyph_id(&mut self, f: FontId, char: char) -> Option<GlyphId> {
		match self.sheet.glyph_id_map.entry((f, char)) {
			Entry::Occupied(r) => Some(r.get().clone()),
			Entry::Vacant(r) => {
				let font = &mut self.sheet.fonts[*f];

				let width = self.brush.width(f, &font.font, char);
				let size = Size {
					width: width, 
					height: font.height};

				// 在纹理中分配一个位置
				let tex_position = self.sheet.text_packer.alloc(
					size.width.ceil() as usize, 
					size.height.ceil() as usize);
				let tex_position = match tex_position {
					Some(r) => r,
					None => return None,
				};

				// 分配GlyphId
				let id = GlyphId(self.sheet.glyphs.insert(GlyphIdDesc{
					font_id: f,
					char,
					glyph: Glyph {
						x: tex_position.x, 
						y: tex_position.y, 
						width: size.width, 
						height: size.height},
				}));

				// 放入等待队列, 并统计等待队列的总宽度
				// font.await_info.size.width += size.width.ceil() as usize;
				// font.await_info.size.height += size.height.ceil() as usize;
				font.await_info.wait_list.push(id);
				
				Some(r.insert(id).clone())
			}
		}
	}

	/// 测量宽度
	fn measure_width(&mut self, f: FontId, char: char) -> f32 {
		match self.sheet.glyph_id_map.entry((f, char)) {
			Entry::Occupied(r) => {
				let id = r.get();
				self.sheet.glyphs[**id].glyph.width
			},
			Entry::Vacant(_r) => {
				let font = &mut self.sheet.fonts[*f];
				self.brush.width(f, &font.font, char)
			}
		}
	}

	/// 取到字形信息
	fn glyph(&self, id: GlyphId) -> &Glyph {
		&self.sheet.glyphs[*id].glyph
	}

	/// 绘制文字
	fn draw(&mut self) {
		self.brush.draw(&self.sheet);
		// 清理等待列表
		for (_k, font_info) in self.fonts.iter_mut() {
			font_info.await_info.wait_list.clear();
			font_info.await_info.size = Size {width: 0, height: 0};// 似乎没有作用？
		}
	}

	/// 取到纹理
	fn texture_view(&self) -> &Handle<RenderRes<TextureView>> {
		&self.sheet.texture_view
	}

	/// 取到纹理版本
	fn texture_version(&self) -> usize {
		self.sheet.texture_version.load(Ordering::Relaxed)
	}

	fn texture_size(&self) -> Size<usize> {
		self.sheet.texture_size()
	}

	/// 清理字形信息
	fn clear(&mut self) {
		self.sheet.fonts.clear();
		self.sheet.fonts_map.clear();
		self.sheet.glyph_id_map.clear();
		self.sheet.glyphs.clear();
		self.sheet.text_packer.clear();
	}
}

pub fn get_size(s: &FontSize) -> usize {
    match s {
        &FontSize::None => {
			// size
			panic!()
		},
        &FontSize::Length(r) => r,
        &FontSize::Percent(_r) => {
			// (r * size as f32).round() as usize;
			panic!()
		},
    }
}

pub const BASE_FONT_SIZE: usize = 32;

pub struct GlyphIdDesc {
	pub font_id: FontId,
	pub char: char,
	pub glyph: Glyph,
}

#[derive(Debug)]
pub struct FontInfo {
	pub font: Font,
	pub height: f32,
	pub await_info: AwaitInfo,
}

#[derive(Debug)]
pub struct AwaitInfo {
	pub size: Size<usize>,
	pub wait_list: Vec<GlyphId>,
	// pub top: usize,
}


#[derive(Debug, Default, Clone)]
pub struct Glyph {
	pub x: usize,
    pub y: usize,
	pub width: f32,
    pub height: f32,
}
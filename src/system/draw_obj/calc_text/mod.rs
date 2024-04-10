// mod text;
mod text_glyph;
// mod text_shadow;
mod text_split;
// mod text_texture;
mod text_sdf2;

use bevy_ecs::prelude::Resource;
use bevy_app::{Plugin, App};
use pi_hal::font::font::FontType;
use pi_render::font::{Size, FontSheet};

// use crate::system::system_set::UiSystemSet;
use self::text_sdf2::Sdf2TextPlugin;
// use self::{text::calc_text, text_texture::calc_text_texture, text_sdf2::Sdf2TextPlugin};


#[derive(Debug, Resource, Default)]
pub struct IsRun(pub bool);

pub struct UiTextPlugin {
	pub font_type: FontType
}

impl Plugin for UiTextPlugin {
    fn build(&self, app: &mut App) {
		// let font_sheet = ShareFontSheet::new(&mut app.world, self.font_type);
        // app.insert_resource(font_sheet)
        //     .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
        //     .add_frame_event::<ComponentEvent<Changed<TextContent>>>()
        //     // 文字劈分
        //     .add_systems(Update, text_split::text_split.before(calc_layout).in_set(UiSystemSet::Layout))
        //     // 字形计算
        //     .add_systems(Update, text_glyph::text_glyph.after(cal_matrix).before(calc_text).in_set(UiSystemSet::Matrix))
            // // 更新文字纹理
            // .add_systems(Update, calc_text_texture.in_set(UiSystemSet::PrepareDrawObj))
			// // 文字drawobj创建
			// .add_systems(Update, 
			// 	draw_object_life::<
			// 		TextContent,
			// 		TextRenderObjType,
			// 		TextMark,
			// 		{ TEXT_ORDER },
			// 	>
			// 		.in_set(UiSystemSet::LifeDrawObject),
			// )// 设置文字的的顶点、索引，和颜色、边框颜色、边框宽度的Uniform
            
            // 文字阴影
            // .add_plugins(UiTextShadowPlugin);
		// match self.font_type {
		// 	// FontType::Bitmap => app.add_systems(Update, 
        //     //     calc_text
        //     //         .in_set(UiSystemSet::PrepareDrawObj)
        //     //         .after(calc_text_texture)
        //     // ),
		// 	// FontType::Sdf1 => app.add_systems(Update, 
        //     //     calc_text
        //     //         .in_set(UiSystemSet::PrepareDrawObj)
        //     //         .after(calc_text_texture)
        //     // ),
		// 	FontType::Sdf2 => app.add_plugins(Sdf2TextPlugin),
        //     _ => (),
		// };
        app.add_plugins(Sdf2TextPlugin);
    }
}


pub const TEXT_ORDER: u8 = 9;

#[derive(Debug, Default)]
pub struct TextureState {
    pub width: usize,
    pub height: usize,
    pub version: usize,
}

impl TextureState {
    // 返回（宽高是否发生改变，版本是否发生改变）
    pub fn is_change(&mut self, size: &Size<usize>, font_sheet: &FontSheet) -> (bool, bool) {
        let size_is_change = size.width != self.width || size.height != self.height;
        if size_is_change {
            self.width = size.width;
            self.height = size.height;
        }
		let font_type = font_sheet.font_mgr().font_type;
       
		let version = match font_type {
			FontType::Bitmap => font_sheet.texture_version(),
			FontType::Sdf1 => font_sheet.sdf_texture_version(),
			FontType::Sdf2 => font_sheet.texture_version(),
		};

		let version_is_change = version != self.version;
		if version_is_change {
			self.version = version;
		}

        (size_is_change, version_is_change)
    }
}

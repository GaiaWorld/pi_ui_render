use std::io::Result;

use bytemuck::{Pod, Zeroable};
use image::DynamicImage;
use pi_assets::asset::{Handle, GarbageEmpty};
use pi_assets::mgr::{AssetMgr, LoadResult, Receiver};
use pi_async_common::MULTI_RUNTIME;
use pi_async::rt::AsyncRuntime;
use pi_atom::Atom;
use pi_ecs::prelude::{Or, Deleted, With, ParamSet, OrDefault};
use pi_ecs::prelude::{Query, Changed, EntityCommands, Commands, Write, Res, Event, Id};
use pi_ecs_macros::{listen, setup};
use pi_multimedia::image::ImageRes;
use pi_render::rhi::RenderQueue;
use pi_render::rhi::asset::{RenderRes, TextureRes};
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_share::{Share, ShareMutex};
use wgpu::IndexFormat;

use crate::components::calc::{LayoutResult, BorderImageTexture};
use crate::components::user::{BorderImageClip, BorderImageSlice, BorderImageRepeat, BorderImage, Polygon, Point2, BorderImageRepeatOption};
use crate::resource::draw_obj::CommonSampler;
use crate::system::shader_utils::StaticIndex;
use crate::system::shader_utils::image::{ImageStaticIndex, PosUvVertexLayout, IMAGE_POSITION_LOCATION, IMAGE_TEXTURE_GROUP};
use crate::utils::tools::{calc_hash};
use crate::{components::{user::Node, calc::{NodeId, DrawList}, draw_obj::{DrawObject, DrawState}}, resource::draw_obj::Shaders};
// use crate::utils::tools::calc_hash;

pub struct CalcBorderImage;

#[setup]
impl CalcBorderImage {
	/// 创建RenderObject，用于渲染背景颜色
	#[system]
	pub async fn calc_border_image(
		mut query: ParamSet<(
			// 布局修改、BorderImage修改、圆角修改或删除，需要修改或创建BorderImage的DrawObject
			Query<Node, (
				Id<Node>, 
				&'static BorderImage,
				&'static BorderImageTexture,
				OrDefault<BorderImageClip>,
				OrDefault<BorderImageSlice>,
				OrDefault<BorderImageRepeat>,
				&'static LayoutResult,
				Write<BorderImageDrawId>, 
				Write<DrawList>,
			), (With<BorderImageTexture>, Or<(
				Changed<BorderImageTexture>,
				Changed<BorderImageClip>,
				Deleted<BorderImageClip>,
				Changed<BorderImageSlice>,
				Deleted<BorderImageSlice>,
				Changed<BorderImageRepeat>,
				Deleted<BorderImageRepeat>,
				Changed<LayoutResult>,
			)>)>,

			// BorderImage删除，需要删除对应的DrawObject
			Query<Node, (
				Option<&'static BorderImageTexture>,
				Write<BorderImageDrawId>,
				Write<DrawList>,
			), Deleted<BorderImageTexture>>
		)>,

		query_draw: Query<DrawObject, Write<DrawState>>,
		mut draw_obj_commands: EntityCommands<DrawObject>,
		mut draw_state_commands: Commands<DrawObject, DrawState>,
		mut node_id_commands: Commands<DrawObject, NodeId>,
		mut shader_static_commands: Commands<DrawObject, StaticIndex>,
		
		// load_mgr: ResMut<'a, LoadMgr>,
		device: Res<'static, RenderDevice>,
		static_index: Res<'static, ImageStaticIndex>,
		vertex_layout: Res<'static, PosUvVertexLayout>,
		shader_static: Res<'static, Shaders>,
		common_sampler: Res<'static, CommonSampler>,

		buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
		bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,
	) -> Result<()> {
		// border image 中的position和uv，完全是一一对应的，几乎不存在，position或uv单独被其他renderObj重用的情况
		// 因此，position和uv的布局不使用默认的布局方式，而是将其放入同一个buffer中
		let mut static_index = (*static_index).clone();
		static_index.vertex_buffer_index = **vertex_layout;
		// log::info!("calc_background=================");
		// TODO: 删除逻辑在个system中重复，需要抽象出去
		for (
			border_image,
			mut draw_index,
			mut render_list) in query.p1_mut().iter_mut() {
			if border_image.is_some() {
				// 可能存在border_image删除后，再创建的情况，跳过该情况
				continue;
			};
			// 删除对应的DrawObject
			if let Some(draw_index_item) = draw_index.get() {
				draw_obj_commands.despawn(draw_index_item.0.clone());
				if let Some(r) = render_list.get_mut() {
					for i in 0..r.len() {
						let item = &r[i];
						if item == &draw_index_item.0 {
							r.swap_remove(i);
						}
					}
				}
				draw_index.remove();
			}
		}

		let texture_group_layout = &shader_static.get(static_index.shader).unwrap().bind_group[IMAGE_TEXTURE_GROUP];
		for (
			node,
			border_image,
			border_texture,
			border_image_clip,
			border_image_slice,
			border_image_repeat,
			layout,
			mut draw_index,
			mut render_list) in query.p0_mut().iter_mut() {

			match draw_index.get() {
				// borderimage已经存在一个对应的DrawObj， 则修改color group
				Some(r) => {
					let mut draw_state_item = query_draw.get_unchecked(**r);
					let draw_state = draw_state_item.get_mut().unwrap();
					modify(
						&border_image, 
						&border_texture, 
						&border_image_clip,
						&border_image_slice,
						&border_image_repeat,
						layout,
						draw_state,
						&device, 
						&buffer_assets,
						&bind_group_assets,
						texture_group_layout,
						&common_sampler).await;
					draw_state_item.notify_modify();
				},
				// 否则，创建一个新的DrawObj，并设置color group; 
				// 修改以下组件：
				// * <Node, BackgroundDrawId>
				// * <Node, DrawList>
				// * <DrawObject, DrawState>
				// * <DrawObject, NodeId>
				// * <DrawObject, IsUnitQuad>
				None => {
					// log::info!("create_background=================");
					// 创建新的DrawObj
					let new_draw_obj = draw_obj_commands.spawn();
					// 设置DrawState（包含color group）
					let mut draw_state = DrawState::default();
					modify(
						&border_image, 
						&border_texture, 
						&border_image_clip,
						&border_image_slice,
						&border_image_repeat,
						layout,
						&mut draw_state,
						&device, 
						&buffer_assets,
						&bind_group_assets,
						texture_group_layout,
						&common_sampler).await;
					
					draw_state_commands.insert(new_draw_obj, draw_state);
					// 建立DrawObj对Node的索引
					node_id_commands.insert(new_draw_obj, NodeId(node));

					shader_static_commands.insert(new_draw_obj, static_index.clone());

					// 建立Node对DrawObj的索引
					draw_index.write(BorderImageDrawId(new_draw_obj));
					
					match render_list.get_mut() {
						Some(r) => {
							r.push(new_draw_obj);
							render_list.notify_modify();
						},
						None => {
							let mut r = DrawList::default();
							r.push(new_draw_obj);
							render_list.write(r);
						},
					};
				}
			}
		}
		return Ok(())
	}
}

#[derive(Clone, DerefMut, Deref)]
pub struct BorderImageAwait(Share<ShareMutex<Vec<(Id<Node>, Atom, Handle<TextureRes>)>>>);

impl Default for BorderImageAwait {
    fn default() -> Self {
        Self(Share::new(ShareMutex::new(Vec::new())))
    }
}

pub struct CalcBorderImageLoad;

#[setup]
impl CalcBorderImageLoad {
	/// BorderImage创建，加载对应的图片
	/// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
	/// 因为BorderImageTexture未加锁，其他线程可能正在使用
	/// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
	#[listen(component=(Node, BorderImage, Create))]
	pub fn border_image_change(
		e: Event,
		mut query: Query<Node, (&BorderImage, Write<BorderImageTexture>)>,
		texture_assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
		image_assets_mgr: Res<Share<AssetMgr<ImageRes>>>,
		border_image_await: Res<BorderImageAwait>,
		queue: Res<RenderQueue>,
		device: Res<RenderDevice>,
	) {
		let (key, mut texture) = query.get_unchecked_mut_by_entity(e.id);
		match AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64)) {
            LoadResult::Ok(r) => texture.write(BorderImageTexture(r)),
            LoadResult::Wait(f) => {
				let awaits = (*border_image_await).clone();
				let (id, key) = (unsafe { Id::new(e.id.local())}, key.clone());
				MULTI_RUNTIME.spawn(MULTI_RUNTIME.alloc(), async move {
					let r = f.await.unwrap();
					awaits.lock().unwrap().push((id, (*key).clone(), r))
				}).unwrap();
			},
            LoadResult::Receiver(recv) => {
				let (awaits, device, queue) =( (*border_image_await).clone(),  (*device).clone(), (*queue).clone());
				let image_assets_mgr = (*image_assets_mgr).clone();
				let (id, key) = (unsafe { Id::new(e.id.local())}, (*key).clone());
				MULTI_RUNTIME.spawn(MULTI_RUNTIME.alloc(), async move {
					let image = pi_multimedia::image::load_from_path(&image_assets_mgr, &*key).await;
					let image = match image {
						Ok(r) => r,
						Err(_) =>  {
							log::error!("load image fail: {:?}", key.as_str());
							panic!();
						},
					};

					let texture = create_texture_from_image(&image, &device, &queue, (*key).clone(), recv).await;
					awaits.lock().unwrap().push((id, (*key).clone(), texture))
				}).unwrap();
            }
        }
	}

	// 
	#[system]
	pub fn check_await_texture(
		border_image_await: Res<BorderImageAwait>,
		mut query: Query<Node, (&BorderImage, Write<BorderImageTexture>)>,

	) {
		let awaits = {
			let mut border_image_await = border_image_await.0.lock().unwrap();
			std::mem::replace(&mut *border_image_await, Vec::new())
		};
		
		for (id, key, texture) in awaits.into_iter() {
			let mut texture_item = match query.get_mut(id) {
				Some((img, texture_item)) => {
					// borderimage已经修改，不需要设置texture
					if **img != key {
						continue;
					}
					texture_item
				},
				// 节点已经销毁，或borderimage已经被删除，不需要设置texture
				None => continue,
			};
			texture_item.write(BorderImageTexture(texture));
		}
	}
}

pub async fn create_texture_from_image(
	image: &Handle<ImageRes>, 
	device: &RenderDevice, 
	queue: &RenderQueue,
	key: Atom,
	recv: Receiver<TextureRes, GarbageEmpty>
) -> Handle<TextureRes> {
	let buffer_temp;
	let buffer_temp1;
	let (width, height, buffer, ty, pre_pixel_size) = match &****image {
		DynamicImage::ImageLuma8(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::R8Unorm, 1),
		DynamicImage::ImageRgb8(r) => {
			buffer_temp =  image.to_rgba8();
			(r.width(), r.height(), buffer_temp.as_raw(), wgpu::TextureFormat::Rgba8UnormSrgb, 4)
		},
		DynamicImage::ImageRgba8(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Rgba8UnormSrgb, 4),
		DynamicImage::ImageBgr8(r) => {
			buffer_temp1 =  image.to_bgra8();
			(r.width(), r.height(), buffer_temp1.as_raw(), wgpu::TextureFormat::Bgra8UnormSrgb, 4)
		},
		DynamicImage::ImageBgra8(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8UnormSrgb, 4),

		_ => panic!("不支持的图片格式"),

		// DynamicImage::ImageLumaA8(image) => panic!("不支持的图片格式: DynamicImage::ImageLumaA8"),
		// DynamicImage::ImageLuma16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8UnormSrgb),
		// DynamicImage::ImageLumaA16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8UnormSrgb),

		// DynamicImage::ImageRgb16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8UnormSrgb),
		// DynamicImage::ImageRgba16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8UnormSrgb),
	};
	let texture_extent = wgpu::Extent3d {
		width,
		height,
		depth_or_array_layers: 1,
	};
	let texture = (**device).create_texture(&wgpu::TextureDescriptor {
		label: Some("first depth buffer"),
		size: texture_extent,
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: ty,
		usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
	});
	let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
	queue.write_texture(
		texture.as_image_copy(),
		buffer,
		wgpu::ImageDataLayout {
			offset: 0,
			bytes_per_row: Some(std::num::NonZeroU32::new(width * pre_pixel_size).unwrap()),
			rows_per_image: None,
		},
		texture_extent,
	);

	recv.receive(key.get_hash() as u64, Ok(TextureRes::new(width, height, (width * height * pre_pixel_size) as usize, texture_view))).await.unwrap()
}

#[derive(Deref, Default)]
pub struct BorderImageDrawId(Id<DrawObject>);

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BorderImageTexture, Delete), component=(Node, Node, Delete))]
pub fn background_color_delete(
	e: Event,
	query: Query<Node, &BorderImageDrawId>,
	mut draw_obj: EntityCommands<DrawObject>,
) {
	if let Some(index) = query.get_by_entity(e.id) {
		draw_obj.despawn(**index);
	}
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

// 返回当前需要的StaticIndex
async fn modify<'a> (
	image: &BorderImage,
	texture: &BorderImageTexture,
	clip: &BorderImageClip,
	slice: &BorderImageSlice,
	repeat: &BorderImageRepeat,
	layout: &LayoutResult,
	draw_state: &mut DrawState, 
	device: &RenderDevice, 
	buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
	group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
	texture_group_layout: &BindGroupLayout,
	common_sampler: &CommonSampler,
) {
	// key TODO
	// &layout.border, &layout.rect
	let buffer_key = calc_hash(&("border image", image, clip, slice, repeat));
	let index_key = calc_hash(&("border image index", image, clip, slice, repeat));

	let (vertex, indices) = get_border_image_stream(
		texture,
		clip,
		slice,
		repeat,
		layout,
		Vec::new(),
		Vec::new(),
	);

	let vertex_buffer = match buffer_assets.get(&buffer_key) {
		Some(r) => r,
		None => {
			let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
				label: Some("border image vert buffer init"),
				contents: bytemuck::cast_slice(&vertex),
				usage: wgpu::BufferUsages::VERTEX
			});
			buffer_assets.cache(buffer_key, RenderRes::new(buf, vertex.len() * 4));
			buffer_assets.get(&buffer_key).unwrap()
		}
	};
	draw_state.vbs.insert(IMAGE_POSITION_LOCATION, (vertex_buffer, 0));

	let index_buffer = match buffer_assets.get(&index_key) {
		Some(r) => r,
		None => {
			let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
				label: Some("border image index buffer init"),
				contents: bytemuck::cast_slice(&indices),
				usage: wgpu::BufferUsages::INDEX
			});
			buffer_assets.cache(index_key, RenderRes::new(buf, indices.len() * 2));
			buffer_assets.get(&index_key).unwrap()
		}
	};

	draw_state.ib = Some((index_buffer, indices.len() as u64, IndexFormat::Uint16));

	// texture BindGroup
	let texture_group = match group_assets.get(&buffer_key) {
		Some(r) => r,
		None => {
			let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: texture_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: wgpu::BindingResource::TextureView(&texture.texture_view),
					},
				],
				label: Some("border image group create"),
			});
			group_assets.cache(buffer_key, RenderRes::new(group, 5));
			group_assets.get(&buffer_key).unwrap().clone()
		},
	};
	draw_state.bind_groups.insert(IMAGE_TEXTURE_GROUP, texture_group);
}


#[inline]
fn get_border_image_stream(
    texture: &BorderImageTexture,
    clip: &BorderImageClip,
    slice: &BorderImageSlice,
    repeat: &BorderImageRepeat,
    layout: &LayoutResult,
    mut vert_arr: Polygon,
    mut index_arr: Vec<u16>,
) -> (Polygon, Vec<u16>) {
	let width = layout.rect.right - layout.rect.left;
	let height = layout.rect.bottom - layout.rect.top;
    let p1 = Point2::new(0.0, 0.0);
    let p2 = Point2::new(width, height);
    let left = layout.border.left;
    let right = width - layout.border.right;
    let top = layout.border.top;
    let bottom = height - layout.border.bottom;
    let uvw = *clip.right - *clip.left;
    let uvh = *clip.bottom - *clip.top;
    let (uv_left, uv_right, uv_top, uv_bottom) = (
		*clip.left + *slice.left * uvw,
		*clip.right - *slice.right * uvw,
		*clip.top + *slice.top * uvh,
		*clip.bottom - *slice.bottom * uvh,
	);

    //  p1, p2, w, h, left, right, top, bottom, "UV::", uv1, uv2, uvw, uvh, uv_left, uv_right, uv_top, uv_bottom);
    // TODO 在仅使用左或上的边框时， 应该优化成8个顶点
    // 先将16个顶点和uv放入数组，记录偏移量
    let mut pi = (vert_arr.len() / 3) as u16;
    // 左上的4个点
    let p_x1_y1 = push_vertex(
        &mut vert_arr,
        p1.x,
        p1.y,
        *clip.left,
        *clip.top,
        &mut pi,
    );
    let p_x1_top = push_vertex(
        &mut vert_arr,
        p1.x,
        top,
        *clip.left,
        uv_top,
        &mut pi,
    );
    let p_left_top = push_vertex(
        &mut vert_arr,
        left,
        top,
        uv_left,
        uv_top,
        &mut pi,
    );
    let p_left_y1 = push_vertex(
        &mut vert_arr,
        left,
        p1.y,
        uv_left,
        *clip.top,
        &mut pi,
    );
    push_quad(&mut index_arr, p_x1_y1, p_x1_top, p_left_top, p_left_y1);

    // 左下的4个点
    let p_x1_bottom = push_vertex(
        &mut vert_arr,
        p1.x,
        bottom,
        *clip.left,
        uv_bottom,
        &mut pi,
    );
    let p_x1_y2 = push_vertex(
        &mut vert_arr,
        p1.x,
        p2.y,
        *clip.left,
        *clip.bottom,
        &mut pi,
    );
    let p_left_y2 = push_vertex(
        &mut vert_arr,
        left,
        p2.y,
        uv_left,
        *clip.bottom,
        &mut pi,
    );
    let p_left_bottom = push_vertex(
        &mut vert_arr,
        left,
        bottom,
        uv_left,
        uv_bottom,
        &mut pi,
    );
    push_quad(
        &mut index_arr,
        p_x1_bottom,
        p_x1_y2,
        p_left_y2,
        p_left_bottom,
    );

    // 右下的4个点
    let p_right_bottom = push_vertex(
        &mut vert_arr,
        right,
        bottom,
        uv_right,
        uv_bottom,
        &mut pi,
    );
    let p_right_y2 = push_vertex(
        &mut vert_arr,
        right,
        p2.y,
        uv_right,
        *clip.bottom,
        &mut pi,
    );
    let p_x2_y2 = push_vertex(
        &mut vert_arr,
        p2.x,
        p2.y,
        *clip.right,
        *clip.bottom,
        &mut pi,
    );
    let p_x2_bottom = push_vertex(
        &mut vert_arr,
        p2.x,
        bottom,
        *clip.right,
        uv_bottom,
        &mut pi,
    );
    push_quad(
        &mut index_arr,
        p_right_bottom,
        p_right_y2,
        p_x2_y2,
        p_x2_bottom,
    );

    // 右上的4个点
    let p_right_y1 = push_vertex(
        &mut vert_arr,
        right,
        p1.y,
        uv_right,
        *clip.top,
        &mut pi,
    );
    let p_right_top = push_vertex(
        &mut vert_arr,
        right,
        top,
        uv_right,
        uv_top,
        &mut pi,
    );
    let p_x2_top = push_vertex(
        &mut vert_arr,
        p2.x,
        top,
        *clip.right,
        uv_top,
        &mut pi,
    );
    let p_x2_y1 = push_vertex(
        &mut vert_arr,
        p2.x,
        p1.y,
        *clip.right,
        *clip.top,
        &mut pi,
    );
    push_quad(&mut index_arr, p_right_y1, p_right_top, p_x2_top, p_x2_y1);

    // 根据图像大小和uv计算
	let ustep = calc_step(right - left, texture.0.width as f32 * (uv_right - uv_left), repeat.0);
	let vstep = calc_step(
		bottom - top,
		texture.0.height as f32 * (uv_bottom - uv_top),
		repeat.1,
	);

	if ustep > 0.0 {
		push_u_arr(
			&mut vert_arr,
			&mut index_arr,
			p_left_y1,
			p_left_top,
			p_right_top,
			p_right_y1,
			uv_left,
			*clip.top,
			uv_right,
			uv_top,
			ustep,
			&mut pi,
		); // 上边
		push_u_arr(
			&mut vert_arr,
			&mut index_arr,
			p_left_bottom,
			p_left_y2,
			p_right_y2,
			p_right_bottom,
			uv_left,
			uv_bottom,
			uv_right,
			*clip.bottom,
			ustep,
			&mut pi,
		); // 下边
	}
	
	if vstep > 0.0 {
		push_v_arr(
			&mut vert_arr,
			&mut index_arr,
			p_x1_top,
			p_x1_bottom,
			p_left_bottom,
			p_left_top,
			*clip.left,
			uv_top,
			uv_left,
			uv_bottom,
			vstep,
			&mut pi,
		); // 左边
		push_v_arr(
			&mut vert_arr,
			&mut index_arr,
			p_right_top,
			p_right_bottom,
			p_x2_bottom,
			p_x2_top,
			uv_right,
			uv_top,
			*clip.right,
			uv_bottom,
			vstep,
			&mut pi,
		); // 右边
	}
	
	// 处理中间
	if slice.fill {
		push_quad(
			&mut index_arr,
			p_left_top,
			p_left_bottom,
			p_right_bottom,
			p_right_top,
		);
	}

    (vert_arr, index_arr)
}
// 将四边形放进数组中
fn push_vertex(
    point_arr: &mut Polygon,
    x: f32,
    y: f32,
    u: f32,
    v: f32,
    i: &mut u16,
) -> u16 {
    point_arr.extend_from_slice(&[x, y]);
	point_arr.extend_from_slice(&[u, v]);
    // uv_arr.extend_from_slice(&[u, v]);
    let r = *i;
    *i += 1;
    r
}
// 将四边形放进数组中
fn push_quad(index_arr: &mut Vec<u16>, p1: u16, p2: u16, p3: u16, p4: u16) {
    index_arr.extend_from_slice(&[p1, p2, p3, p1, p3, p4]);
}

// 根据参数计算uv的step
fn calc_step(csize: f32, img_size: f32, rtype: BorderImageRepeatOption) -> f32 {
    let c = csize / img_size;
    if c <= 1.0 {
        return std::f32::INFINITY;
    }
    match rtype {
        BorderImageRepeatOption::Repeat => csize / c.round(),
        BorderImageRepeatOption::Round => csize / c.ceil(),
        BorderImageRepeatOption::Space => csize / c.floor(),
        _ => std::f32::INFINITY,
    }
}

// 将指定区域按u切开
fn push_u_arr(
    point_arr: &mut Polygon,
    index_arr: &mut Vec<u16>,
    p1: u16,
    p2: u16,
    p3: u16,
    p4: u16,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    step: f32,
    i: &mut u16,
) {
    let y1 = point_arr[p1 as usize * 2 + 1];
    let y2 = point_arr[p2 as usize * 2 + 1];
    let mut cur = point_arr[p1 as usize * 2] + step;
    let max = point_arr[p3 as usize * 2];
    let mut pt1 = p1;
    let mut pt2 = p2;
    while cur < max {
        let i3 = push_vertex(point_arr, cur, y2, u2, v2, i);
        let i4 = push_vertex(point_arr, cur, y1, u2, v1, i);
        push_quad(index_arr, pt1, pt2, i3, i4);
        // 因为uv不同，新插入2个顶点
        pt1 = push_vertex(point_arr, cur, y1, u1, v1, i);
        pt2 = push_vertex(point_arr, cur, y2, u1, v2, i);
        cur += step;
    }
    push_quad(index_arr, pt1, pt2, p3, p4);
}
// 将指定区域按v切开
fn push_v_arr(
    point_arr: &mut Polygon,
    index_arr: &mut Vec<u16>,
    p1: u16,
    p2: u16,
    p3: u16,
    p4: u16,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    step: f32,
    i: &mut u16,
) {
    let x1 = point_arr[p1 as usize * 2];
    let x2 = point_arr[p4 as usize * 2];
    let mut cur = point_arr[p1 as usize * 2 + 1] + step;
    let max = point_arr[p3 as usize * 2 + 1];
    let mut pt1 = p1;
    let mut pt4 = p4;
    while cur < max {
        let i2 = push_vertex(point_arr, x1, cur, u1, v2, i);
        let i3 = push_vertex(point_arr, x2, cur, u2, v2, i);
        push_quad(index_arr, pt1, i2, i3, pt4);
        // 因为uv不同，新插入2个顶点
        pt1 = push_vertex(point_arr, x1, cur, u1, v1, i);
        pt4 = push_vertex(point_arr, x2, cur, u2, v1, i);
        cur += step;
    }
    push_quad(index_arr, pt1, p2, p3, pt4);
}


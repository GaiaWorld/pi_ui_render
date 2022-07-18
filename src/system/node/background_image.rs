use std::io::Result;

use ordered_float::NotNan;
use pi_assets::asset::{Handle, Asset};
use pi_assets::mgr::AssetMgr;
use pi_atom::Atom;
use pi_ecs::prelude::{Deleted, Or, ParamSet, With, OrDefault};
use pi_ecs::prelude::{Changed, Commands, EntityCommands, Event, Id, Query, Res, Write};
use pi_ecs_macros::{listen, setup};
use pi_flex_layout::prelude::Rect;
use pi_polygon::{interp_mult_by_lg, mult_to_triangle, split_by_lg, split_by_radius,
    split_mult_by_lg, LgCfg,
};
use pi_render::rhi::asset::{RenderRes, TextureRes};
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::bind_group_layout::BindGroupLayout;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_share::{Share, ShareMutex};
use wgpu::IndexFormat;

use crate::components::calc::{BackgroundImageTexture, LayoutResult};
use crate::components::draw_obj::BoxType;
use crate::components::user::{
    Aabb2, BackgroundImageClip, BorderRadius, FitType, ObjectFit, Point2, Vector2, Size,
};
use crate::resource::draw_obj::CommonSampler;
use crate::system::shader_utils::image::{
    ImageStaticIndex, IMAGE_POSITION_LOCATION, IMAGE_TEXTURE_GROUP, IMAGE_UV_LOCATION,
};

use crate::system::shader_utils::StaticIndex;
use crate::utils::tools::{calc_hash, get_content_radius, calc_float_hash};
use crate::{
    components::{
        calc::{DrawList, NodeId},
        draw_obj::{DrawObject, DrawState},
        user::{BackgroundImage, Node},
    },
    resource::draw_obj::{Shaders, UnitQuadBuffer},
};

pub struct CalcBackgroundImage;

#[setup]
impl CalcBackgroundImage {
    /// 创建RenderObject，用于渲染背景颜色
    #[system]
    pub async fn calc_background_image(
        mut query: ParamSet<(
            // 布局修改、BackgroundImage修改、BackgroundImageClip修改、圆角修改或删除，需要修改或创建背景图片的DrawObject
            Query<
                Node,
                (
                    Id<Node>,
                    &'static BackgroundImage,
                    Option<&'static BorderRadius>,
                    &'static LayoutResult,
                    Write<BackgroundImageDrawId>,
                    Write<DrawList>,
					OrDefault<BackgroundImageClip>,
                    OrDefault<ObjectFit>,
                    &'static BackgroundImageTexture,
                ),
                (
                    With<BackgroundImageTexture>,
                    Or<(
                        Changed<BackgroundImageTexture>,
                        Changed<BorderRadius>,
                        Deleted<BorderRadius>,
                        Changed<BackgroundImageClip>,
                        Changed<LayoutResult>,
                    )>,
                ),
            >,
            // BackgroundImage删除，需要删除对应的DrawObject
            Query<
                Node,
                (
                    Option<&'static BackgroundImage>,
                    Write<BackgroundImageDrawId>,
                    Write<DrawList>,
                ),
                Deleted<BackgroundImage>,
            >,
        )>,

        query_draw: Query<DrawObject, (Write<DrawState>, OrDefault<BoxType>)>,
        mut draw_obj_commands: EntityCommands<DrawObject>,
        mut draw_state_commands: Commands<DrawObject, DrawState>,
        mut node_id_commands: Commands<DrawObject, NodeId>,
        mut shader_static_commands: Commands<DrawObject, StaticIndex>,
		mut is_unit_quad_commands: Commands<DrawObject, BoxType>,

        // load_mgr: ResMut<'a, LoadMgr>,
        device: Res<'static, RenderDevice>,
        static_index: Res<'static, ImageStaticIndex>,
        shader_static: Res<'static, Shaders>,
        unit_quad_buffer: Res<'static, UnitQuadBuffer>,
        common_sampler: Res<'static, CommonSampler>,

        buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
        bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,
    ) -> Result<()> {
        // log::info!("calc_background================= image");
        for (background_image, mut draw_index, mut render_list) in query.p1_mut().iter_mut() {
            // BackgroundColor不存在时，删除对应DrawObject
            if background_image.is_some() {
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
        let texture_group_layout =
            &shader_static.get(static_index.shader).unwrap().bind_group[IMAGE_TEXTURE_GROUP];

        for (
            node,
            background_image,
            radius,
            layout,
            mut draw_index,
            mut render_list,
            background_image_clip_change,
            fit,
            background_image_texture,
        ) in query.p0_mut().iter_mut()
        {
            match draw_index.get() {
                // background_color已经存在一个对应的DrawObj， 则修改color group
                Some(r) => {
                    let (mut draw_state_item, old_unit_quad) = query_draw.get_unchecked(**r);
                    let draw_state = draw_state_item.get_mut().unwrap();
                    let new_unit_quad = modify(
                        &background_image,
                        radius,
                        layout,
                        draw_state,
                        &device,
                        &buffer_assets,
                        &unit_quad_buffer,
                        &bind_group_assets,
                        &texture_group_layout,
                        &background_image_texture,
                        background_image_clip_change,
                        fit,
                        &common_sampler,
                    )
                    .await;
                    draw_state_item.notify_modify();

                    if *old_unit_quad != new_unit_quad {
                        is_unit_quad_commands.insert(**r, new_unit_quad);
                    }
                }
                // 否则，创建一个新的DrawObj，并设置color group;
                // 修改以下组件：
                // * <Node, BackgroundImageDrawId>
                // * <Node, DrawList>
                // * <DrawObject, DrawState>
                // * <DrawObject, NodeId>
                // * <DrawObject, IsUnitQuad>
                None => {
                    // 创建新的DrawObj
                    let new_draw_obj = draw_obj_commands.spawn();
                    // 设置DrawState（包含color group）
                    let mut draw_state = DrawState::default();
                    let new_unit_quad = modify(
                        &background_image,
                        radius,
                        layout,
                        &mut draw_state,
                        &device,
                        &buffer_assets,
                        &unit_quad_buffer,
                        &bind_group_assets,
                        &texture_group_layout,
                        &background_image_texture,
                        background_image_clip_change,
                        fit,
                        &common_sampler,
                    )
                    .await;
                    draw_state_commands.insert(new_draw_obj, draw_state);
                    // 建立DrawObj对Node的索引
                    node_id_commands.insert(new_draw_obj, NodeId(node));
                    is_unit_quad_commands.insert(new_draw_obj, new_unit_quad);
                    shader_static_commands.insert(new_draw_obj, static_index.clone());

                    // 建立Node对DrawObj的索引
                    draw_index.write(BackgroundImageDrawId(new_draw_obj));
                    match render_list.get_mut() {
                        Some(r) => {
                            r.push(new_draw_obj);
                            render_list.notify_modify();
                        }
                        None => {
                            let mut r = DrawList::default();
                            r.push(new_draw_obj);
                            render_list.write(r);
                        }
                    };
                }
            }
        }
        return Ok(());
    }
}

#[derive(Deref, Default)]
pub struct BackgroundImageDrawId(Id<DrawObject>);

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BackgroundImage, Delete), component=(Node, Node, Delete))]
pub fn background_image_delete(
    e: Event,
    query: Query<Node, &BackgroundImageDrawId>,
    mut draw_obj: EntityCommands<DrawObject>,
) {
    if let Some(index) = query.get_by_entity(e.id) {
        draw_obj.despawn(**index);
    }
}

// 返回当前需要的StaticIndex
async fn modify<'a>(
    image: &BackgroundImage,
    radius: Option<&BorderRadius>,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    unit_quad_buffer: &UnitQuadBuffer,
    group_assets: &Share<AssetMgr<RenderRes<BindGroup>>>,
    texture_group_layout: &BindGroupLayout,
    texture: &BackgroundImageTexture,
    clip: &BackgroundImageClip,
    fit: &ObjectFit,
    common_sampler: &CommonSampler,
) -> BoxType {

    let radius = get_content_radius(radius, layout);

	let (vertex_buffer, uv_buffer, index_buffer, is_unit) = if radius.is_some() || fit.0 != FitType::Fill {
		let (pos, uv_aabb) = get_pos_uv(texture, clip, fit, layout);
		// modify_radius_linear_geo
		let vertex_key = calc_float_hash(&[pos.mins.x, pos.mins.y, pos.maxs.x, pos.maxs.y,uv_aabb.mins.x, uv_aabb.mins.y, uv_aabb.maxs.x, uv_aabb.maxs.y,], calc_hash(&("radius vert", radius), 0));
		let uv_key =  calc_float_hash(&[pos.mins.x, pos.mins.y, pos.maxs.x, pos.maxs.y,uv_aabb.mins.x, uv_aabb.mins.y, uv_aabb.maxs.x, uv_aabb.maxs.y,], calc_hash(&("radius uv", radius), 0));
		let index_key = calc_float_hash(&[pos.mins.x, pos.mins.y, pos.maxs.x, pos.maxs.y,uv_aabb.mins.x, uv_aabb.mins.y, uv_aabb.maxs.x, uv_aabb.maxs.y,], calc_hash(&("radius index", radius), 0));

		
		let (mut vertex, mut uv, mut indices) = (
			vec![
				pos.mins.x, pos.mins.y, pos.maxs.x, pos.mins.y, pos.maxs.x, pos.maxs.y, pos.mins.x,
				pos.maxs.y,
			],
			vec![
				uv_aabb.mins.x,
				uv_aabb.mins.y,
				uv_aabb.maxs.x,
				uv_aabb.mins.y,
				uv_aabb.maxs.x,
				uv_aabb.maxs.y,
				uv_aabb.mins.x,
				uv_aabb.maxs.y,
			],
			vec![0, 1, 2, 2, 3, 0],
		);

		let v_buffer = match buffer_assets.get(&vertex_key) {
			Some(r) => r,
			None => {
				if radius.is_some() {
					(vertex, uv, indices) = use_layout_pos(uv_aabb, layout, radius.as_ref().unwrap());
				}
	
				let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
					label: Some("background image vert buffer init"),
					contents: bytemuck::cast_slice(&vertex),
					usage: wgpu::BufferUsages::VERTEX,
				});
				buffer_assets.insert(vertex_key, RenderRes::new(buf, vertex.len() * 4)).unwrap()
			}
		};
		let uv_buffer = match buffer_assets.get(&uv_key) {
			Some(r) => r,
			None => {
	
				let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
					label: Some("background image uv buffer init"),
					contents: bytemuck::cast_slice(&uv),
					usage: wgpu::BufferUsages::VERTEX,
				});
				buffer_assets.insert(uv_key, RenderRes::new(buf, uv.len() * 2)).unwrap()
			}
		};
		let index_buffer = match buffer_assets.get(&index_key) {
			Some(r) => r,
			None => {
				let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
					label: Some("background image index buffer init"),
					contents: bytemuck::cast_slice(&indices),
					usage: wgpu::BufferUsages::INDEX,
				});
				buffer_assets.insert(index_key, RenderRes::new(buf, indices.len() * 2)).unwrap()
			}
		};
		(v_buffer, uv_buffer, index_buffer, BoxType::None)
	} else {
		(unit_quad_buffer.vertex.clone(), unit_quad_buffer.vertex.clone(), unit_quad_buffer.index.clone(), BoxType::Content)
	};

    draw_state
        .vbs
        .insert(IMAGE_POSITION_LOCATION, (vertex_buffer, 0));
    draw_state.vbs.insert(IMAGE_UV_LOCATION, (uv_buffer, 0));
	let len = index_buffer.size() / 2;
    draw_state.ib = Some((index_buffer, len as u64, IndexFormat::Uint16));

	let texture_group_key = calc_hash(&image.0.get_hash(), calc_hash(&"image texture", 0));
    // texture BindGroup
    let texture_group = match group_assets.get(&texture_group_key) {
        Some(r) => r,
        None => {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: texture_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&common_sampler.default),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture.texture_view),
                    },
                ],
                label: Some("border image group create"),
            });
            group_assets.insert(texture_group_key, RenderRes::new(group, 5)).unwrap()
        }
    };

    draw_state
        .bind_groups
        .insert(IMAGE_TEXTURE_GROUP, texture_group);

	is_unit
}

#[derive(Clone, DerefMut, Deref)]
pub struct BackgroundImageAwait(Share<ShareMutex<Vec<(Id<Node>, Atom, Handle<TextureRes>)>>>);

impl Default for BackgroundImageAwait {
    fn default() -> Self {
        Self(Share::new(ShareMutex::new(Vec::new())))
    }
}

// pub struct CalcBackgroundImageLoad;

// #[setup]
// impl CalcBackgroundImageLoad {
//     /// BorderImage创建，加载对应的图片
//     /// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
//     /// 因为BorderImageTexture未加锁，其他线程可能正在使用
//     /// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
//     #[listen(component=(Node, BackgroundImage, Create))]
//     pub fn background_image_change(
//         e: Event,
//         mut query: Query<Node, (&BackgroundImage, Write<BackgroundImageTexture>)>,
//         texture_assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
//         image_assets_mgr: Res<Share<AssetMgr<ImageRes>>>,
//         border_image_await: Res<BackgroundImageAwait>,
//         queue: Res<RenderQueue>,
//         device: Res<RenderDevice>,
//     ) {
//         println!("=============== background_image_change");
//         let (key, mut texture) = query.get_unchecked_mut_by_entity(e.id);
//         match AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64)) {
//             LoadResult::Ok(r) => texture.write(BackgroundImageTexture(r)),
//             LoadResult::Wait(f) => {
//                 let awaits = (*border_image_await).clone();
//                 let (id, key) = (unsafe { Id::new(e.id.local()) }, key.clone());
//                 MULTI_RUNTIME
//                     .spawn(MULTI_RUNTIME.alloc(), async move {
//                         let r = f.await.unwrap();
//                         awaits.lock().unwrap().push((id, (*key).clone(), r))
//                     })
//                     .unwrap();
//             }
//             LoadResult::Receiver(recv) => {
//                 let (awaits, device, queue) = (
//                     (*border_image_await).clone(),
//                     (*device).clone(),
//                     (*queue).clone(),
//                 );
//                 let image_assets_mgr = (*image_assets_mgr).clone();
//                 let (id, key) = (unsafe { Id::new(e.id.local()) }, (*key).clone());
//                 MULTI_RUNTIME
//                     .spawn(MULTI_RUNTIME.alloc(), async move {
//                         let image =
//                             pi_multimedia::image::load_from_path(&image_assets_mgr, &*key).await;
//                         let image = match image {
//                             Ok(r) => r,
//                             Err(_) => {
//                                 log::error!("load image fail: {:?}", key.as_str());
//                                 panic!();
//                             }
//                         };

//                         let texture = create_texture_from_image(
//                             &image,
//                             &device,
//                             &queue,
//                             (*key).clone(),
//                             recv,
//                         )
//                         .await;
//                         awaits.lock().unwrap().push((id, (*key).clone(), texture))
//                     })
//                     .unwrap();
//             }
//         }
//     }

//     //
//     #[system]
//     pub fn check_await_texture(
//         border_image_await: Res<BackgroundImageAwait>,
//         mut query: Query<Node, (&BackgroundImage, Write<BackgroundImageTexture>)>,
//     ) {
//         let awaits = {
//             let mut border_image_await = border_image_await.0.lock().unwrap();
//             std::mem::replace(&mut *border_image_await, Vec::new())
//         };

//         for (id, key, texture) in awaits.into_iter() {
//             let mut texture_item = match query.get_mut(id) {
//                 Some((img, texture_item)) => {
//                     // borderimage已经修改，不需要设置texture
//                     if **img != key {
//                         continue;
//                     }
//                     texture_item
//                 }
//                 // 节点已经销毁，或borderimage已经被删除，不需要设置texture
//                 None => continue,
//             };
//             println!("=========== texture_item write");
//             texture_item.write(BackgroundImageTexture(texture));
//             println!("=========== texture_item write end");
//         }
//     }
// }

// 获得图片的4个点(逆时针)的坐标和uv的Aabb
fn get_pos_uv(
    img: &BackgroundImageTexture,
    clip: &BackgroundImageClip,
    fit: &ObjectFit,
    layout: &LayoutResult,
) -> (Aabb2, Aabb2) {
    let src = img.0.as_ref();
	let size = Vector2::new(
		src.width as f32 * (clip.maxs.x - clip.mins.x).abs(),
		src.height as f32 * (clip.maxs.y - clip.mins.y).abs(),
	);
	let (mut uv1, mut uv2) = (clip.mins, clip.maxs);

    let width = layout.rect.right - layout.rect.left - layout.border.right - layout.border.left;
    let height = layout.rect.bottom - layout.rect.top - layout.border.bottom - layout.border.top;

    let mut p1 = Point2::new(
        layout.border.left + layout.padding.left,
        layout.border.top + layout.padding.top,
    );
    let mut p2 = Point2::new(width, height);
    let w = p2.x - p1.x;
    let h = p2.y - p1.y;
    // 如果不是填充，总是居中显示。 如果在范围内，则修改点坐标。如果超出的部分，会进行剪切，剪切会修改uv坐标。
	match fit.0 {
		FitType::None => {
			// 保持原有尺寸比例。同时保持内容原始尺寸大小。 超出部分会被剪切
			if size.x <= w {
				let x = (w - size.x) / 2.0;
				p1.x += x;
				p2.x -= x;
			} else {
				let x = (size.x - w) * (uv2.x - uv1.x) * 0.5 / size.x;
				uv1.x += x;
				uv2.x -= x;
			}
			if size.y <= h {
				let y = (h - size.y) / 2.0;
				p1.y += y;
				p2.y -= y;
			} else {
				let y = (size.y - h) * (uv2.y - uv1.y) * 0.5 / size.y;
				uv1.y += y;
				uv2.y -= y;
			}
		}
		FitType::Contain => {
			// 保持原有尺寸比例。保证内容尺寸一定可以在容器里面放得下。因此，此参数可能会在容器内留下空白。
			fill(&size, &mut p1, &mut p2, w, h);
		}
		FitType::Cover => {
			// 保持原有尺寸比例。保证内容尺寸一定大于容器尺寸，宽度和高度至少有一个和容器一致。超出部分会被剪切
			let rw = size.x / w;
			let rh = size.y / h;
			if rw > rh {
				let x = (size.x - w * rh) * (uv2.x - uv1.x) * 0.5 / size.x;
				uv1.x += x;
				uv2.x -= x;
			} else {
				let y = (size.y - h * rw) * (uv2.y - uv1.y) * 0.5 / size.y;
				uv1.y += y;
				uv2.y -= y;
			}
		}
		FitType::ScaleDown => {
			// 如果内容尺寸小于容器尺寸，则直接显示None。否则就是Contain
			if size.x <= w && size.y <= h {
				let x = (w - size.x) / 2.0;
				let y = (h - size.y) / 2.0;
				p1.x += x;
				p1.y += y;
				p2.x -= x;
				p2.y -= y;
			} else {
				fill(&size, &mut p1, &mut p2, w, h);
			}
		}
		FitType::Repeat => panic!("TODO"),  // TODO
		FitType::RepeatX => panic!("TODO"), // TODO
		FitType::RepeatY => panic!("TODO"), // TODO
		FitType::Fill => (),                // 填充。 内容拉伸填满整个容器，不保证保持原有的比例
	};
    (
        Aabb2 { mins: p1, maxs: p2 },
        Aabb2 {
            mins: uv1,
            maxs: uv2,
        },
    )
}

// 按比例缩放到容器大小，居中显示
fn fill(size: &Vector2, p1: &mut Point2, p2: &mut Point2, w: f32, h: f32) {
    let rw = size.x / w;
    let rh = size.y / h;
    if rw > rh {
        let y = (h - size.y / rw) / 2.0;
        p1.y += y;
        p2.y -= y;
    } else {
        let x = (w - size.x / rh) / 2.0;
        p1.x += x;
        p2.x -= x;
    }
}

fn use_layout_pos(
    uv: Aabb2,
    layout: &LayoutResult,
    radius: &Rect<NotNan<f32>>,
) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
    let width = layout.rect.right - layout.rect.left;
    let height = layout.rect.bottom - layout.rect.top;
    let start_x = layout.border.left;
    let start_y = layout.border.top;
    let end_x = width - layout.border.right;
    let end_y = height - layout.border.bottom;
    let (positions, indices) = if *radius.left == 0.0 || width == 0.0 || height == 0.0 {
        (
            vec![
                start_x, start_y, start_x, end_y, end_x, end_y, end_x, start_y,
            ],
            vec![0, 1, 2, 3],
        )
    } else {
        split_by_radius(
            start_x,
            start_y,
            end_x - start_x,
            end_y - start_y,
            *radius.left - start_x,
            None,
        )
    };
    // debug_println!("indices: {:?}", indices);
    // debug_println!("split_by_lg,  positions:{:?}, indices:{:?}, top_percent: {}, bottom_percent: {}, start: ({}, {}) , end: ({}, {})", positions, indices, 0.0, 1.0, 0.0, 0.0, 0.0, layout.height);
    let (positions, indices_arr) =
        split_by_lg(positions, indices, &[0.0, 1.0], (0.0, 0.0), (0.0, height));
    // debug_println!("split_mult_by_lg, positions: {:?}, indices_arr: {:?}, cfg: {:?}, percent: [{}, {}], start: [{}, {}], end: [{}, {}]",  &positions, indices_arr, vec![LgCfg{unit: 1, data: vec![uv.min.x, uv.max.x]}], 0.0, 1.0, 0.0, 0.0, layout.width, 0.0);
    let (positions, indices_arr) = split_mult_by_lg(
        positions,
        indices_arr,
        &[0.0, 1.0],
        (0.0, 0.0),
        (width, 0.0),
    );
    let indices = mult_to_triangle(&indices_arr, Vec::new());
    // debug_println!("u positions: {:?}, indices_arr: {:?}, cfg: {:?}, percent: [{}, {}], start: [{}, {}], end: [{}, {}]",  &positions, indices_arr, vec![LgCfg{unit: 1, data: vec![uv.min.x, uv.max.x]}], 0.0, 1.0, 0.0, 0.0, layout.width, 0.0);
    let u = interp_mult_by_lg(
        &positions,
        &indices_arr,
        vec![Vec::new()],
        vec![LgCfg {
            unit: 1,
            data: vec![uv.mins.x, uv.maxs.x],
        }],
        &[0.0, 1.0],
        (0.0, 0.0),
        (width, 0.0),
    );
    let v = interp_mult_by_lg(
        &positions,
        &indices_arr,
        vec![Vec::new()],
        vec![LgCfg {
            unit: 1,
            data: vec![uv.mins.y, uv.maxs.y],
        }],
        &[0.0, 1.0],
        (0.0, 0.0),
        (0.0, height),
    );
    // debug_println!("v positions: {:?}, indices_arr: {:?}, cfg: {:?}, percent: [}, {}], start: [{}, {}], end: [{}, {}]",  &positions, indices_arr, vec![LgCfg{unit: 1, data: vec![uv.min.y, uv.max.y]}], 0.0, 1.0, 0.0, 0.0, 0.0, layout.height);
    let mut uvs = Vec::with_capacity(u[0].len());
    for i in 0..u[0].len() {
        uvs.push(u[0][i]);
        uvs.push(v[0][i]);
    }

    (positions, uvs, indices)
    // render_obj.geometry = Some(engine.create_geo_res(
    //     0,
    //     indices.as_slice(),
    //     &[
    //         AttributeDecs::new(AttributeName::Position, positions.as_slice(), 2),
    //         AttributeDecs::new(AttributeName::UV0, uvs.as_slice(), 2),
    //     ],
    // ));
}

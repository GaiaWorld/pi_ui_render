use std::io::Result;

use ordered_float::NotNan;
use pi_assets::asset::Handle;
use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::res::WriteRes;
use pi_ecs::prelude::{ChangeTrackers, Deleted, Local, Or, OrDefault, ParamSet, ResMut, With};
use pi_ecs::prelude::{Changed, Commands, EntityCommands, Event, Id, Query, Res, Write};
use pi_ecs_macros::{listen, setup};
use pi_polygon::{find_lg_endp, interp_mult_by_lg, mult_to_triangle, split_by_lg, LgCfg};
use pi_render::font::{FontSheet, Glyph, GlyphId};
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::buffer::Buffer;
use pi_render::rhi::device::RenderDevice;
use pi_render::rhi::dyn_uniform_buffer::{Bind, Group};
use pi_share::{Share, ShareCell};
use smallvec::smallvec;
use wgpu::IndexFormat;

use crate::components::calc::{DrawInfo, LayoutResult, NodeState};
use crate::components::draw_obj::{DrawGroup, DynDrawGroup};
use crate::components::user::{CgColor, TextContent, TextStyle};
use crate::resource::draw_obj::{
    CommonSampler, DynBindGroupIndex, DynUniformBuffer, EmptyVertexBuffer, StaticIndex, TextStaticIndex, TextTextureGroup,
};
use crate::shaders::text::{
    PositionVertexBuffer, SampTex2DGroup, StrokeColorUniform, TextMaterialBind, TextMaterialGroup, TextureSizeUniform, UcolorUniform, UvVertexBuffer,
};
use crate::utils::tools::{calc_hash, calc_hash_slice};
use crate::{
    components::{
        calc::{DrawList, NodeId},
        draw_obj::{DrawObject, DrawState, FSDefines, VSDefines},
        user::{BackgroundColor, Color, Node},
    },
    resource::draw_obj::Shaders,
};

pub struct CalcText;

// pub struct TextShareBuffer {
// 	size_buffer: Buffer,
// 	size: Handle<RenderRes<BindGroup>>,
// 	texture_group: Handle<RenderRes<BindGroup>>,
// 	empty_gradient_vert_buffer: Handle<RenderRes<Buffer>>,
// }

// impl FromWorld for TextShareBuffer {
//     fn from_world(world: &mut pi_ecs::prelude::World) -> Self {
// 		world.get_or_insert_resource::<CommonSampler,
// 		>();
// 		let common_sampler = world.get_resource::<CommonSampler,
// 		>().unwrap();
//         let device = world.get_resource::<RenderDevice>().unwrap();
// 		let bind_group_assets = world.get_resource::<Share<AssetMgr<RenderRes<BindGroup>>>>().unwrap();
// 		let buffer_assets = world.get_resource::<Share<AssetMgr<RenderRes<Buffer>>>>().unwrap();
// 		let text_static_index = world.get_resource::<TextStaticIndex>().unwrap();
// 		let shader_static = world.get_resource::<Shaders,
// 		>().unwrap();
// 		let font_sheet = world.get_resource::<Share<ShareCell<FontSheet<>>>,
// 		>().unwrap();
// 		let font_sheet = font_sheet.borrow();

// 		let size = font_sheet.texture_size();
// 		let buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
// 			label: Some("Text TextrueSize  Buffer"),
// 			contents: bytemuck::cast_slice(&[size.width as f32, size.height as f32]),
// 			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
// 		});

// 		let size_group_layout = &shader_static.get(text_static_index.shader).unwrap().bind_group_layout[TEXT_TEXTURE_SIZE_GROUP];

// 		let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
// 			layout: &size_group_layout,
// 			entries: &[
// 				wgpu::BindGroupEntry {
// 					binding: 0,
// 					resource: buf.as_entire_binding(),
// 				},
// 			],
// 			label: Some("Text TextrueSize group create"),
// 		});

// 		let key = calc_hash(&"TEXT_TEXTURE_SIZE_GROUP", 0);
// 		bind_group_assets.insert(key, RenderRes::new(group, 5)).unwrap();

// 		let texture_group_layout = &shader_static.get(text_static_index.shader).unwrap().bind_group_layout[SampTex2DGroup::id() as usize];
// 		let texture_group_key = calc_hash(&"TEXT TETURE", 0);
// 		let texture_group = match bind_group_assets.get(&texture_group_key) {
// 			Some(r) => r,
// 			None => {
// 				let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
// 					layout: texture_group_layout,
// 					entries: &[
// 						wgpu::BindGroupEntry {
// 							binding: 0,
// 							resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
// 						},
// 						wgpu::BindGroupEntry {
// 							binding: 1,
// 							resource: wgpu::BindingResource::TextureView(&font_sheet.texture_view().texture_view),
// 						},
// 					],
// 					label: Some("post process texture bind group create"),
// 				});
// 				bind_group_assets.insert(texture_group_key, RenderRes::new(group, 5)).unwrap()
// 			},
// 		};


// 		let default_color_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
// 			label: Some("Text Default Color Buffer"),
// 			contents: bytemuck::cast_slice(&[1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32]),
// 			usage: wgpu::BufferUsages::UNIFORM,
// 		});

// 		Self {
// 			size_buffer: buf,
// 			size: bind_group_assets.get(&key).unwrap(),
// 			texture_group,
// 			empty_gradient_vert_buffer: gradient_buf,
// 		}
//     }
// }

#[setup]
impl CalcText {
    /// 创建RenderObject，用于渲染文字
    #[system]
    pub async fn calc_text(
        mut query: ParamSet<(
            // 布局修改、文字属性，需要修改或创建背景色的DrawObject
            Query<
                'static,
                'static,
                Node,
                (
                    Id<Node>,
                    &'static NodeState,
                    &'static LayoutResult,
                    OrDefault<TextStyle>,
                    Write<TextDrawId>,
                    Write<DrawList>,
                    ChangeTrackers<TextContent>,
                    ChangeTrackers<NodeState>,
                ),
                (
                    With<TextContent>,
                    Or<(Changed<TextStyle>, Deleted<TextContent>, Changed<NodeState>)>,
                ),
            >,
            // TextContent删除，需要删除对应的DrawObject
            Query<'static, 'static, Node, (Option<&'static TextContent>, Write<TextDrawId>, Write<DrawList>), Deleted<TextContent>>,
        )>,

        mut query_draw: Query<'static, 'static, DrawObject, (Write<DrawState>, &'static mut VSDefines, &'static mut FSDefines)>,
        mut draw_obj_commands: EntityCommands<DrawObject>,
        mut draw_state_commands: Commands<DrawObject, DrawState>,
        mut node_id_commands: Commands<DrawObject, NodeId>,
        mut shader_static_commands: Commands<DrawObject, StaticIndex>,
        mut vs_defines_commands: Commands<DrawObject, VSDefines>,
        mut fs_defines_commands: Commands<DrawObject, FSDefines>,
        mut order_commands: Commands<DrawObject, DrawInfo>,

        // load_mgr: ResMut<'a, LoadMgr>,
        device: Res<'static, RenderDevice>,
        static_index: Res<'static, TextStaticIndex>,
        shader_static: Res<'static, Shaders>,

        font_sheet: Res<'static, Share<ShareCell<FontSheet>>>,

        buffer_assets: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
        bind_group_assets: Res<'static, Share<AssetMgr<RenderRes<BindGroup>>>>,
        mut text_texture_group: WriteRes<'static, TextTextureGroup>,
        empty_vert_buffer: Res<'static, EmptyVertexBuffer>,
        texture_size: Local<'static, (usize, usize)>,
        mut dyn_uniform_buffer: ResMut<'static, DynUniformBuffer>,
        text_material_bind_group: Res<'static, DynBindGroupIndex<TextMaterialGroup>>,
        common_sampler: Res<'static, CommonSampler>,
    ) -> Result<()> {
        // log::info!("calc_background=================");
        let font_sheet = font_sheet.borrow();

        // 更新纹理尺寸
        let _version = font_sheet.texture_version();
        let size = font_sheet.texture_size();
        // if version != *texture_version {
        // 	// let size = font_sheet.texture_size();
        // 	// queue.write_buffer(&text_share.size_buffer, 0, bytemuck::cast_slice(&[size.width as f32, size.height as f32]));
        // 	*texture_version = version;
        // }

        // 纹理大小不同，需要重新创建bind_group
        if size.width != texture_size.0 || size.height != texture_size.1 {
            let texture_group_layout = &shader_static.get(static_index.shader).unwrap().bind_group_layout[SampTex2DGroup::id() as usize];
            let texture_group_key = calc_hash(&("TEXT TETURE", size.width, size.height), 0);
            let texture_group = match bind_group_assets.get(&texture_group_key) {
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
                                resource: wgpu::BindingResource::TextureView(&font_sheet.texture_view().texture_view),
                            },
                        ],
                        label: Some("post process texture bind group create"),
                    });
                    bind_group_assets.insert(texture_group_key, RenderRes::new(group, 5)).unwrap()
                }
            };
            text_texture_group.write(TextTextureGroup(texture_group));
        }

        let texture_group = text_texture_group.get().unwrap();

        for (text_content, mut draw_index, mut render_list) in query.p1_mut().iter_mut() {
            // TextContent不存在时，删除对应DrawObject
            if text_content.is_some() {
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

        for (node, node_state, layout, text_style, mut draw_index, mut render_list, text_change, node_state_change) in query.p0_mut().iter_mut() {
			if node_state.0.scale < 0.000001 {
				continue;
			}
            match draw_index.get() {
                // text已经存在一个对应的DrawObj， 则修改color group
                Some(r) => {
                    let (mut draw_state_item, mut vs_defines, mut fs_defines) = query_draw.get_unchecked_mut(**r);
                    let draw_state = draw_state_item.get_mut().unwrap();
                    modify(
                        &font_sheet,
                        &node_state,
                        &text_style,
                        layout,
                        draw_state,
                        &device,
                        &buffer_assets,
                        &text_change,
                        &node_state_change,
                        &mut vs_defines,
                        &mut fs_defines,
                        &mut 100,
                        node_state.0.scale,
                        &mut dyn_uniform_buffer,
                    );
                    draw_state_item.notify_modify();

                    // 为了触发pipeline重新编译
                    shader_static_commands.insert(**r, static_index.clone());
                }
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
                    let mut vs_defines = VSDefines::default();
                    let mut fs_defines = FSDefines::default();
                    // 设置DrawState（包含color group）
                    let mut draw_state = DrawState::default();

                    let text_material_dyn_offset = dyn_uniform_buffer.alloc_binding::<TextMaterialBind>();
                    dyn_uniform_buffer.set_uniform(&text_material_dyn_offset, &TextureSizeUniform(&[size.width as f32, size.height as f32]));
                    let group = DrawGroup::Dyn(DynDrawGroup::new(
                        (*text_material_bind_group).clone(),
                        smallvec![text_material_dyn_offset],
                    ));
                    draw_state.bind_groups.insert_group(TextMaterialGroup::id(), group);
                    draw_state
                        .bind_groups
                        .insert_group(SampTex2DGroup::id(), DrawGroup::Static((**texture_group).clone()));

                    draw_state.vbs.insert(2, ((*empty_vert_buffer).clone(), 0));

                    modify(
                        &font_sheet,
                        &node_state,
                        &text_style,
                        layout,
                        &mut draw_state,
                        &device,
                        &buffer_assets,
                        &text_change,
                        &node_state_change,
                        &mut vs_defines,
                        &mut fs_defines,
                        &mut 100,
                        node_state.0.scale,
                        &mut dyn_uniform_buffer,
                    );
                    draw_state_commands.insert(new_draw_obj, draw_state);
                    // 建立DrawObj对Node的索引
                    node_id_commands.insert(new_draw_obj, NodeId(node));

                    shader_static_commands.insert(new_draw_obj, static_index.clone());
                    vs_defines_commands.insert(new_draw_obj, vs_defines);
                    fs_defines_commands.insert(new_draw_obj, fs_defines);
                    order_commands.insert(new_draw_obj, DrawInfo(1));

                    // 建立Node对DrawObj的索引
                    draw_index.write(TextDrawId(new_draw_obj));

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

pub struct GradientOrRadius;

#[derive(Deref, Default)]
pub struct TextDrawId(Id<DrawObject>);

// 背景颜色 ShaderInfo的索引
#[derive(Deref, Clone, Debug)]
pub struct BackgroundStaticIndex {
    pub color: StaticIndex,
}

pub const COLOR_GROUP: usize = 4;

/// 实体删除，背景颜色删除时，删除对应的DrawObject
#[listen(component=(Node, BackgroundColor, Delete), component=(Node, Node, Delete))]
pub fn text_delete(e: Event, query: Query<Node, &TextDrawId>, mut draw_obj: EntityCommands<DrawObject>) {
    if let Some(index) = query.get_by_entity(e.id) {
        draw_obj.despawn(**index);
    }
}

// struct Info {
// 	is_quad: IsUnitQuad,
// 	vb: xxx
// }
// 返回当前需要的StaticIndex
fn modify<'a>(
    font_sheet: &FontSheet,
    node_state: &NodeState,
    text_style: &TextStyle,
    layout: &LayoutResult,
    draw_state: &mut DrawState,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    text_change: &ChangeTrackers<TextContent>,
    node_state_change: &ChangeTrackers<NodeState>,
    vs_defines: &mut VSDefines,
    fs_defines: &mut FSDefines,
    index_buffer_max_len: &mut usize,
    scale: f32,
    dyn_uniform_buffer: &mut DynUniformBuffer,
) {
    // 修改vert buffer
    let is_change_geo = match &text_style.color {
        // 如果是rgba颜色，只有当文字内容、文字布局修改时，或上一次为渐变色时，才会重新计算顶点流
        Color::RGBA(_) => {
            if text_change.is_changed() || node_state_change.is_changed() || vs_defines.get("VERTEX_COLOR").is_some() {
                true
            } else {
                false
            }
        }
        // 如果是渐变色，无论当前是修改了文字内容、颜色、还是布局，都必须重新计算顶点流
        Color::LinearGradient(_) => true,
    };

    // 如果顶点流需要重新计算，则修改顶点流
    if is_change_geo {
        modify_geo(
            node_state,
            draw_state,
            layout,
            &text_style.color,
            font_sheet,
            device,
            index_buffer_max_len,
            buffer_assets,
            scale,
            text_style.text_stroke.width,
        );
    }


    // 修改color_group
    let color_temp;
    let color = match &text_style.color {
        Color::RGBA(c) => {
            vs_defines.remove("VERTEX_COLOR");
            fs_defines.remove("VERTEX_COLOR");
            c
        }
        Color::LinearGradient(_) => {
            vs_defines.insert("VERTEX_COLOR".to_string());
            fs_defines.insert("VERTEX_COLOR".to_string());
            color_temp = CgColor::default();
            &color_temp
        }
    };

    let color_temp;
    let stroke = if *text_style.text_stroke.width > 0.0 {
        &text_style.text_stroke.color
    } else {
        color_temp = CgColor::new(0.0, 0.0, 0.0, 0.0);
        &color_temp
    };

    let buffer = &[color.x, color.y, color.z, color.w, stroke.x, stroke.y, stroke.z, stroke.w];
    let dyn_offset = draw_state
        .bind_groups
        .get_group(TextMaterialGroup::id())
        .unwrap()
        .get_offset(TextMaterialBind::index())
        .unwrap();
    dyn_uniform_buffer.set_uniform(dyn_offset, &UcolorUniform(&[color.x, color.y, color.z, color.w]));
    dyn_uniform_buffer.set_uniform(dyn_offset, &StrokeColorUniform(&[stroke.x, stroke.y, stroke.z, stroke.w]));

    if *text_style.text_stroke.width > 0.0 {
        fs_defines.insert("STROKE".to_string());
    } else {
        fs_defines.remove("STROKE");
    }
}

//
#[inline]
fn modify_geo(
    node_state: &NodeState,
    draw_state: &mut DrawState,
    layout: &LayoutResult,
    color: &Color,
    font_sheet: &FontSheet,
    device: &RenderDevice,
    index_buffer_max_len: &mut usize,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
    scale: f32,
    stroke_width: NotNan<f32>,
) {
    let rect = &layout.rect;
    let (mut positions, mut uvs) = text_vert(node_state, layout, font_sheet, scale, stroke_width);
    match color {
        Color::RGBA(_) => {
            // 更新ib
            let l = positions.len() / 8;
            while l > *index_buffer_max_len {
                *index_buffer_max_len = l + 50;
            }
            let index_buffer = get_or_create_index_buffer(*index_buffer_max_len, device, buffer_assets);
            draw_state.ib = Some((index_buffer, (l * 6) as u64, IndexFormat::Uint16));
        }
        Color::LinearGradient(color) => {
            let mut i = 0;
            let mut colors = vec![Vec::new()];
            let mut indices = Vec::with_capacity(6);

            let endp = match node_state.0.is_vnode() {
                // 如果是虚拟节点，则节点自身的布局信息会在顶点上体现，此时找渐变端点需要考虑布局结果的起始点
                true => find_lg_endp(
                    &[
                        rect.left,
                        rect.top,
                        rect.left,
                        rect.bottom,
                        rect.right,
                        rect.bottom,
                        rect.right,
                        rect.top, //渐变端点
                    ],
                    color.direction,
                ),
                // 非虚拟节点，顶点总是以0，0作为起始点，布局起始点体现在世界矩阵上
                false => find_lg_endp(
                    &[
                        0.0,
                        0.0,
                        0.0,
                        rect.bottom - rect.top,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        rect.right - rect.left,
                        0.0,
                    ],
                    color.direction,
                ),
            };

            let mut lg_pos = Vec::with_capacity(color.list.len());
            let mut lg_color = Vec::with_capacity(color.list.len() * 4);
            for v in color.list.iter() {
                lg_pos.push(v.position);
                lg_color.extend_from_slice(&[v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w]);
            }
            let lg_color = vec![LgCfg { unit: 4, data: lg_color }];

            let len = positions.len() / 2;
            let mut old_len = positions.len();
            while (i as usize) < len {
                // log::info!("position: {:?}, {:?}, {:?}, {:?}, {:?}", positions, node_state.0.text, lg_pos, &endp.0, &endp.1);
                let (ps, indices_arr) = split_by_lg(positions, vec![i, i + 1, i + 2, i + 3], lg_pos.as_slice(), endp.0.clone(), endp.1.clone());
                positions = ps;

                // 颜色插值
                colors = interp_mult_by_lg(
                    positions.as_slice(),
                    &indices_arr,
                    colors,
                    lg_color.clone(),
                    lg_pos.as_slice(),
                    endp.0.clone(),
                    endp.1.clone(),
                );

                // 尝试为新增的点计算uv
                if positions.len() > old_len {
                    fill_uv(&mut positions, &mut uvs, i as usize, old_len);
                    old_len = positions.len();
                }

                indices = mult_to_triangle(&indices_arr, indices);
                i = i + 4;
            }

            let index_buffer = get_or_create_buffer_index(bytemuck::cast_slice(&indices), "text vert index buffer", device, buffer_assets);
            draw_state.ib = Some((index_buffer, indices.len() as u64, IndexFormat::Uint16));

            let colors = colors.pop().unwrap();
            let color_buffer = get_or_create_buffer(bytemuck::cast_slice(&colors), "text vert color buffer", device, buffer_assets);
            draw_state.vbs.insert(2, (color_buffer, 0));
        }
    }
    let positions_buffer = get_or_create_buffer(bytemuck::cast_slice(&positions), "text position buffer", device, buffer_assets);
    let uv_buffer = get_or_create_buffer(bytemuck::cast_slice(&uvs), "text uv buffer", device, buffer_assets);
    draw_state.vbs.insert(PositionVertexBuffer::id() as usize, (positions_buffer, 0));
    draw_state.vbs.insert(UvVertexBuffer::id() as usize, (uv_buffer, 0));
}

#[inline]
fn fill_uv(positions: &mut Vec<f32>, uvs: &mut Vec<f32>, i: usize, start: usize) {
    let pi = i * 2;
    let uvi = i * 2;
    let len = positions.len() - pi;
    let (p1, p4) = ((positions[pi], positions[pi + 1]), (positions[pi + 4], positions[pi + 5]));
    let (u1, u4) = ((uvs[uvi], uvs[uvi + 1]), (uvs[uvi + 4], uvs[uvi + 5]));
    if len > 8 {
        let mut i = start;
        while i < positions.len() {
            let pos_x = positions[i];
            let pos_y = positions[i + 1];
            let uv;
            if (pos_x - p1.0).abs() < 0.001 {
                let base = p4.1 - p1.1;
                let ratio = if base == 0.0 { 0.0 } else { (pos_y - p1.1) / (p4.1 - p1.1) };
                uv = (u1.0, u1.1 * (1.0 - ratio) + u4.1 * ratio);
            } else if (pos_x - p4.0).abs() < 0.001 {
                let base = p4.1 - p1.1;
                let ratio = if base == 0.0 { 0.0 } else { (pos_y - p1.1) / (p4.1 - p1.1) };
                uv = (u4.0, u1.1 * (1.0 - ratio) + u4.1 * ratio);
            } else if (pos_y - p1.1).abs() < 0.001 {
                let base = p4.0 - p1.0;
                let ratio = if base == 0.0 { 0.0 } else { (pos_x - p1.0) / (p4.0 - p1.0) };
                uv = (u1.0 * (1.0 - ratio) + u4.0 * ratio, u1.1);
            } else {
                // }else if pos_y == p4.1{
                let base = p4.0 - p1.0;
                let ratio = if base == 0.0 { 0.0 } else { (pos_x - p1.0) / (p4.0 - p1.0) };
                uv = (u1.0 * (1.0 - ratio) + u4.0 * ratio, u4.1);
            }
            uvs.push(uv.0);
            uvs.push(uv.1);
            i += 2;
        }
    }
}

fn get_or_create_index_buffer(count: usize, device: &RenderDevice, buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>) -> Handle<RenderRes<Buffer>> {
    let key = calc_hash(&count, calc_hash(&"index", 0));
    match buffer_assets.get(&key) {
        Some(r) => r,
        None => {
            let mut index_data: Vec<u16> = Vec::with_capacity(count * 6);
            let mut i: u16 = 0;
            while (i as usize) < count * 6 {
                index_data.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
                i += 4;
            }

            let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                label: Some("text color index buffer init"),
                contents: bytemuck::cast_slice(&index_data),
                usage: wgpu::BufferUsages::INDEX,
            });
            buffer_assets.insert(key, RenderRes::new(uniform_buf, index_data.len() * 2)).unwrap()
        }
    }
}

fn get_or_create_buffer(
    buffer: &[u8],
    label: &'static str,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
) -> Handle<RenderRes<Buffer>> {
    let key = calc_hash_slice(buffer, calc_hash(&"vert", 0));
    match buffer_assets.get(&key) {
        Some(r) => r,
        None => {
            let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: buffer,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
            buffer_assets.insert(key, RenderRes::new(uniform_buf, buffer.len())).unwrap()
        }
    }
}

fn get_or_create_buffer_index(
    buffer: &[u8],
    label: &'static str,
    device: &RenderDevice,
    buffer_assets: &Share<AssetMgr<RenderRes<Buffer>>>,
) -> Handle<RenderRes<Buffer>> {
    let key = calc_hash_slice(buffer, calc_hash(&"index", 0));
    match buffer_assets.get(&key) {
        Some(r) => r,
        None => {
            let uniform_buf = device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: buffer,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });
            buffer_assets.insert(key, RenderRes::new(uniform_buf, buffer.len())).unwrap()
        }
    }
}

fn text_vert(node_state: &NodeState, layout: &LayoutResult, font_sheet: &FontSheet, scale: f32, stroke_width: NotNan<f32>) -> (Vec<f32>, Vec<f32>) {
    let mut positions = Vec::new();
    let mut uvs = Vec::new();

    let mut word_pos = (0.0, 0.0);
    let mut count = 0;
    let half_stroke = *stroke_width / 2.0;
    for c in node_state.0.text.iter() {
        if c.ch == char::from(0) {
            if c.count > 0 {
                word_pos = (c.pos.left, c.pos.top);
                count = c.count - 1;
            }
            continue;
        }
        if c.ch <= ' ' {
            continue;
        }

		// log::warn!("glyph!!!==================={:?}, {:?}", c.ch_id, c.ch);
        let glyph = font_sheet.glyph(GlyphId(c.ch_id));
        if count > 0 {
            count -= 1;
            push_pos_uv(
                &mut positions,
                &mut uvs,
                word_pos.0 + c.pos.left - half_stroke,
                word_pos.1 + c.pos.top,
                &glyph,
                c.pos.right - c.pos.left,
                c.pos.bottom - c.pos.top,
                scale,
            );
        } else {
            push_pos_uv(
                &mut positions,
                &mut uvs,
                c.pos.left - half_stroke,
                c.pos.top,
                &glyph,
                c.pos.right - c.pos.left,
                c.pos.bottom - c.pos.top,
                scale,
            );
        }
    }
    (positions, uvs)
}

fn push_pos_uv(positions: &mut Vec<f32>, uvs: &mut Vec<f32>, x: f32, mut y: f32, glyph: &Glyph, width: f32, height: f32, scale: f32) {
    // let font_ratio = width/glyph.width;
    let w = glyph.width / scale;
    let h = glyph.height / scale;

    // height为行高， 当行高高于字体高度时，需要居中
    y += (height - h) / 2.0;
    let left_top = (
        (x * scale).round() / scale,
        (y * scale).round() / scale, // 保证顶点对应整数像素
    );
    let right_bootom = (left_top.0 + w, left_top.1 + h);

    let ps = [
        left_top.0,
        left_top.1,
        left_top.0,
        right_bootom.1,
        right_bootom.0,
        right_bootom.1,
        right_bootom.0,
        left_top.1,
    ];

    let gx = glyph.x as f32;
    let gy = glyph.y as f32;
    let uv = [gx, gy, gx, gy + glyph.height, gx + glyph.width, gy + glyph.height, gx + glyph.width, gy];
    uvs.extend_from_slice(&uv);
    // log::warn!("uv=================={:?}, {:?}, w:{:?},h:{:?},scale:{:?},glyph:{:?}", uv, ps, width, height, scale, glyph);
    positions.extend_from_slice(&ps[..]);
}

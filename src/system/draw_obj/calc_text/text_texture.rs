use pi_world::query::{Or, With};
use pi_world::system::{Local, Query, SingleRes};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::res::{OrInitSingleRes, OrInitSingleResMut};
use pi_bevy_render_plugin::PiRenderDevice;
use pi_hal::font::font::FontType;
use pi_render::renderer::draw_obj::DrawBindGroup;
use pi_render::rhi::asset::RenderRes;
use pi_render::rhi::bind_group::BindGroup;
use pi_render::rhi::shader::BindLayout;

use crate::components::draw_obj::{TextMark, TextShadowMark};
use crate::resource::draw_obj::{CommonSampler, ProgramMetaRes, TextTextureGroup};
use crate::resource::ShareFontSheet;
use crate::shader::text::{ProgramMeta, SampBind};
use crate::shader::ui_meterial::{TextureSizeOrBottomLeftBorderUniform, DataTexSizeUniform};
// use crate::shaders::text::{
//     PositionVertexBuffer, SampTex2DGroup, StrokeColorUniform, TextMaterialBind, TextMaterialGroup, TextureSizeUniform, UcolorUniform, UvVertexBuffer,
// };
use crate::components::draw_obj::DrawState;
use crate::utils::tools::calc_hash;

use super::TextureState;
use super::IsRun;

/// 如果纹理大小发生改变， 为文字纹理创建bind_group， 并重新设置每个文字DrawObject的纹理bind_group
pub fn calc_text_texture(
    mut texture_state: Local<TextureState>,
    mut query: Query<&mut DrawState, (With<TextMark>, With<TextShadowMark>)>,
    mut text_texture_group: OrInitSingleResMut<TextTextureGroup>,
    font_sheet: SingleRes<ShareFontSheet>,
    shader_static: OrInitSingleRes<ProgramMetaRes<ProgramMeta>>,
    bind_group_assets: SingleRes<ShareAssetMgr<RenderRes<BindGroup>>>,
    device: SingleRes<PiRenderDevice>,
    common_sampler: SingleRes<CommonSampler>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}
    let font_sheet = font_sheet.borrow();
    let size = font_sheet.texture_size();
    let (size_is_change, version_is_change) = texture_state.is_change(&size, &font_sheet);
	let font_type =  font_sheet.font_mgr().font_type;
    // 纹理大小不同，需要重新创建bind_group
    if size_is_change || text_texture_group.0.is_none() {
        let texture_group_layout = &shader_static.bind_group_layout[SampBind::set() as usize];
        let texture_group_key = calc_hash(&("TEXT TETURE", size.width, size.height), 0);
        let texture_group = match bind_group_assets.get(&texture_group_key) {
            Some(r) => r,
            None => {
				let view = match font_type {
					
					pi_hal::font::font::FontType::Bitmap => &font_sheet.texture_view().as_ref().unwrap().texture_view,
					pi_hal::font::font::FontType::Sdf1 => &font_sheet.sdf_texture_view().as_ref().unwrap().texture_view,
					pi_hal::font::font::FontType::Sdf2 => &font_sheet.sdf2_index_texture_view.as_ref().unwrap().texture_view,
				};
                let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: texture_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(view),
                        },
                    ],
                    label: Some("post process texture bind group create"),
                });
                bind_group_assets.insert(texture_group_key, RenderRes::new(group, 5)).unwrap()
            }
        };
        text_texture_group.0 = Some(texture_group.clone());
		

		if let FontType::Sdf2 = font_type {
			let data_texture_size = font_sheet.font_mgr().table.sdf2_table.data_packer_size();
			let texture_group_key = calc_hash(&("TEXT DATA TETURE", data_texture_size.width, data_texture_size.height), 0);
			let texture_group = match bind_group_assets.get(&texture_group_key) {
				Some(r) => r,
				None => {
					let view = &font_sheet.sdf2_data_texture_view.as_ref().unwrap().texture_view;
					let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
						layout: texture_group_layout,
						entries: &[
							wgpu::BindGroupEntry {
								binding: 0,
								resource: wgpu::BindingResource::Sampler(&common_sampler.pointer),
							},
							wgpu::BindGroupEntry {
								binding: 1,
								resource: wgpu::BindingResource::TextureView(view),
							},
						],
						label: Some("post process texture bind group create"),
					});
					bind_group_assets.insert(texture_group_key, RenderRes::new(group, 5)).unwrap()
				}
			};
			text_texture_group.1 = Some(texture_group.clone());
		}
    }
	
    // 纹理大小改变或内容改变
    if size_is_change || version_is_change {
        let texture_group = text_texture_group.0.as_ref().unwrap().clone();
        // 纹理大小改变，重新设置所有文字的纹理
        for mut draw_state in query.iter_mut() {
            draw_state
                .bindgroups
                .insert_group(SampBind::set(), DrawBindGroup::Independ(texture_group.clone()));
            draw_state
                .bindgroups
                .set_uniform(&TextureSizeOrBottomLeftBorderUniform(&[size.width as f32, size.height as f32]));

			if let FontType::Sdf2 = font_type {
				// let data_texture_size = font_sheet.font_mgr().table.sdf2_table.data_packer_size();
				draw_state
                .bindgroups
                .set_uniform(&DataTexSizeUniform(&[size.width as f32, size.height as f32]));
			}
        }
    }
}

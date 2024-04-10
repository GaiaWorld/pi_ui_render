//! 处理BlendMode组件
//! BlendMode组件修改时， 设置pipeline的状态
//! BlendMode组件删除时， 设置恢复pipeline的状态到默认值

use bevy_ecs::{
    query::Changed, system::{Query, Commands, Res},
    prelude::{Or, RemovedComponents},
};
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};
use pi_bevy_render_plugin::PiRenderDevice;

use crate::{components::draw_obj::Pipeline, resource::draw_obj::InstanceContext};
use crate::{
    components::{calc::DrawList, user::BlendMode},
    resource::draw_obj::CommonBlendState,
};
use pi_style::style::BlendMode as BlendMode1;

use super::calc_text::IsRun;

/// 计算DrawObj的BlendState
pub fn calc_drawobj_blendstate(
    mut blend_mod_removes: RemovedComponents<BlendMode>,
    query_node: Query<(&BlendMode, &DrawList), Or<(Changed<BlendMode>, Changed<DrawList>)>>,
    query_node1: Query<(Option<&BlendMode>, &DrawList)>,
	mut instances: OrInitResMut<InstanceContext>,
	device: Res<PiRenderDevice> ,
	mut cmds: Commands,
	r: OrInitRes<IsRun>,
) {
	if r.0 {
		return;
	}
    // 删除BlendMode时， 将BlendState恢复到默认值
    for remove_blend in blend_mod_removes.iter() {
        if let Ok((blend_mode, draw_list)) = query_node1.get(remove_blend) {
            if let None = blend_mode {
                for draw_id in draw_list.iter() {
                    cmds.entity(draw_id.id).remove::<Pipeline>();
                }
            }
        }
    }

    // 根据blend_mode设置blend_state
    for (blend_mode, draw_list) in query_node.iter() {
        if draw_list.len() == 0 {
            continue;
        }

        let blend_state = to_blend_state((**blend_mode).clone());
		let pipeline = instances.get_or_create_pipeline(&device, blend_state);
        for draw_id in draw_list.iter() {
			cmds.entity(draw_id.id).insert(Pipeline(pipeline.clone()));
        }
    }
}

fn to_blend_state(blend_mode: BlendMode1) -> wgpu::BlendState {
    match blend_mode {
        BlendMode1::Normal => CommonBlendState::NORMAL,
        BlendMode1::AlphaAdd => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        },

        BlendMode1::Subtract => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        },
        BlendMode1::Multiply => CommonBlendState::PREMULTIPLY,
        BlendMode1::OneOne => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        },
    }
}

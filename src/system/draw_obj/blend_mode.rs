//! 处理BlendMode组件
//! BlendMode组件修改时， 设置pipeline的状态
//! BlendMode组件删除时， 设置恢复pipeline的状态到默认值


use pi_world::filter::Or;
use pi_world::param_set::ParamSet;
use pi_world::prelude::{Changed, SingleRes, Alter, Query};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::PiRenderDevice;

use crate::components::calc::{style_bit, StyleBit, StyleMarkType};
use crate::resource::{GlobalDirtyMark, IsRun, OtherDirtyType};
use crate::{components::draw_obj::Pipeline, resource::draw_obj::InstanceContext};
use crate::{
    components::{calc::DrawList, user::BlendMode},
    resource::draw_obj::CommonBlendState,
};
use pi_style::style::{BlendMode as BlendMode1, StyleType};

/// 计算DrawObj的BlendState
pub fn calc_drawobj_blendstate(
    query_node: Query<(&BlendMode, &DrawList), Or<(Changed<BlendMode>, Changed<DrawList>)>>,
    // blend_mod_removes: Query<(&DrawList, Has<BlendMode>)>,
    // removed: ComponentRemoved<BlendMode>, // 操作指令不会删除BlendMode， 只会改为默认值

	mut instances: OrInitSingleResMut<InstanceContext>,
	device: SingleRes<PiRenderDevice> ,
	mut cmds: ParamSet<(
        Alter<(), (), (), (Pipeline,)>,
        Alter<(), (), (Pipeline,), ()>,
    )>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
    // // 删除BlendMode时， 将BlendState恢复到默认值
    // for removed_id in removed.iter() {
    //     if let Ok((draw_list, has_blend_mode)) = blend_mod_removes.get(*removed_id) {
    //         if !has_blend_mode {
    //             for draw_id in draw_list.iter() {
    //                 let _ = cmds.p0().alter(draw_id.id, ());
    //             }
    //         }
    //     }
    // }
    

    // 根据blend_mode设置blend_state
    for (blend_mode, draw_list) in query_node.iter() {
        if draw_list.len() == 0 {
            continue;
        }

        let blend_state = to_blend_state((**blend_mode).clone());
		let pipeline = instances.get_or_create_pipeline(&device, blend_state, true);
        for draw_id in draw_list.iter() {
			let _ = cmds.p1().alter(draw_id.id, (Pipeline(pipeline.clone()), ));
            // log::warn!("blend==========={:?}", (entity, draw_id.id, blend_mode));
        }
        instances.rebatch = true; // 需要重新批处理
    }
}

lazy_static! {
	pub static ref BLEN_DMOD_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::BlendMode as usize)
		.set_bit(OtherDirtyType::DrawObjCreate as usize);
}

pub fn blend_mod_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*BLEN_DMOD_DIRTY)
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

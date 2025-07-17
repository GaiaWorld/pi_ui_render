//! 处理BlendMode组件
//! BlendMode组件修改时， 设置pipeline的状态
//! BlendMode组件删除时， 设置恢复pipeline的状态到默认值

use pi_world::filter::{Changed, Or, With};
use pi_world::param_set::ParamSet;
use pi_world::prelude::{SingleRes, Alter, Query};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::PiRenderDevice;

use crate::components::calc::{style_bit, StyleBit, StyleMarkType};
use crate::components::user::Canvas;
use crate::resource::{CanvasRenderObjType, GlobalDirtyMark, IsRun, OtherDirtyType};
use crate::{components::draw_obj::Pipeline, resource::draw_obj::InstanceContext};
use crate::{
    components::{calc::DrawList, user::BlendMode},
    resource::draw_obj::CommonBlendState,
};
use pi_style::style::{BlendMode as BlendMode1, StyleType};

/// 计算DrawObj的BlendState
pub fn calc_drawobj_pilepine(
    // 过滤条件中不需要加上Changed<Canvas>, 只有Canvas创建时， 需要重建Canvas的Pipeline， Canvas创建， DrawList必然改变
    query_node: Query<(Option<&BlendMode>, Option<&Canvas>, &DrawList), (Or<(With<BlendMode>, With<Canvas>)>, Or<(Changed<BlendMode>, Changed<DrawList>)>)>,
    // blend_mod_removes: Query<(&DrawList, Has<BlendMode>)>,
    // removed: ComponentRemoved<BlendMode>, // 操作指令不会删除BlendMode， 只会改为默认值

    // 在批处理时， 默认仅根据不透明和半透明选择Pipeline
    // blend_mod和Canvas会改变默认的Pipeline
    // blend_mod_changed: ComponentChanged<BlendMode>,
    // canvas_changed: ComponentAdded<Canvas>,

	mut instances: OrInitSingleResMut<InstanceContext>,
	device: SingleRes<PiRenderDevice> ,
	mut cmds: ParamSet<(
        Alter<(), (), (), (Pipeline,)>,
        Alter<(), (), (Pipeline,), ()>,
    )>,
	r: OrInitSingleRes<IsRun>,
    canvas_type: OrInitSingleRes<CanvasRenderObjType>,
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
    for (blend_mode, canvas, draw_list) in query_node.iter() {
        if draw_list.len() == 0 {
            continue;
        }

        let blend_mode = match blend_mode {
            Some(r) => r.0,
            None => pi_style::style::BlendMode::Normal,
        };

        let blend_state = to_blend_state((blend_mode).clone());
        for draw_id in draw_list.iter() {
            let (transparent_pipeline, opacity_pipeline) = if draw_id.ty == ***canvas_type{
                (
                    instances.get_or_create_pipeline(&device, blend_state, true, true, false),
                    instances.get_or_create_pipeline(&device, blend_state, true, true, true)
                )
            } else {
                (
                    instances.get_or_create_pipeline(&device, blend_state, true, false, false),
                    instances.get_or_create_pipeline(&device, blend_state, true, false, true),
                )
            };
			let _ = cmds.p1().alter(draw_id.id, (Pipeline {
                transparent: transparent_pipeline,
                opacity: opacity_pipeline,
            }, ));
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

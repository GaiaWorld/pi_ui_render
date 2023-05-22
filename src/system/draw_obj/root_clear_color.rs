use bevy::ecs::{
    prelude::RemovedComponents,
    query::Changed,
    system::{Query, Res},
};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::res::{OrInitRes, OrInitResMut};
use pi_bevy_render_plugin::PiRenderDevice;
use pi_render::{
    renderer::draw_obj::DrawBindGroup,
    rhi::{asset::RenderRes, bind_group::BindGroup},
};

use crate::{
    components::{
        draw_obj::ClearColorBindGroup,
        user::{ClearColor, Matrix4},
    },
    resource::draw_obj::{DepthCache, ShareGroupAlloter, UiMaterialGroup},
    shader::ui_meterial::{ColorUniform, WorldUniform},
};

// 清屏颜色修改后，重新创建bindgroup
#[allow(unused_must_use)]
pub fn clear_change(
    mut query: Query<(&ClearColor, &mut ClearColorBindGroup), Changed<ClearColor>>,
    mut dels: RemovedComponents<ClearColor>,
    ui_meterial_alloter: OrInitRes<ShareGroupAlloter<UiMaterialGroup>>,
    // color_material_bind_group: Res<DynBindGroupIndex<ColorMaterialGroup>>,
    mut depth_cache: OrInitResMut<DepthCache>,
    device: Res<PiRenderDevice>,
    bind_group_assets: Res<ShareAssetMgr<RenderRes<BindGroup>>>,
) {
    // 处理清屏颜色删除
    for del in dels.iter() {
        if let Ok((_color, mut color_bind_group)) = query.get_mut(del) {
            color_bind_group.0 = None;
        }
    }

    // 处理清屏颜色修改
    for (color, mut color_bind_group) in query.iter_mut() {
        let color_bind_group = match &mut color_bind_group.0 {
            Some(r) => r,
            None => {
                // 深度设置为-1(最远)
                depth_cache.or_create_depth(0, &device, &bind_group_assets);

                let mut ui_group = ui_meterial_alloter.alloc();
                // 世界矩阵
                let world = Matrix4::new(2.0, 0.0, 0.0, -1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
                ui_group.set_uniform(&WorldUniform(world.as_slice()));

                color_bind_group.0 = Some((DrawBindGroup::Offset(ui_group), DrawBindGroup::Independ(depth_cache.list[0].clone())));
                color_bind_group.0.as_mut().unwrap()
            }
        };
        color_bind_group
            .0
            .set_uniform(&ColorUniform(&[color.0.x, color.0.y, color.0.z, color.0.w]));
    }
}

// pub fn transform_will_change_post_process(
//     query_matrix: Query<(&'static WorldMatrix, &'static LayoutResult)>
// ) {
// }

use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut};
use pi_style::style::StyleType;
use pi_world::{event::ComponentChanged, fetch::Ticker, query::Query, schedule_config::IntoSystemConfigs, single_res::SingleRes};
use crate::{components::{calc::{RenderContextMark, WorldMatrix}, pass_2d::WorldMatrixInvert, user::{ClipPath, TransformWillChange}}, resource::{GlobalDirtyMark, OtherDirtyType, RenderContextMarkType}, system::system_set::UiSystemSet};
use pi_world::prelude::{Plugin, App};
use crate::prelude::UiStage;

pub struct WorldInvertPlugin;
impl Plugin for WorldInvertPlugin {
    fn build(&self, app: &mut App) {
        app
            // TransformWillChange标记为需要计算世界矩阵的逆矩阵
            .add_startup_system(UiStage, mark_world_invert::<TransformWillChange, {StyleType::TransformWillChange as usize}>)
            // ClipPath标记为需要计算世界矩阵的逆矩阵
            .add_startup_system(UiStage, mark_world_invert::<ClipPath, {StyleType::ClipPath as usize}>)
            .add_system(UiStage, calc_world_invert.after(UiSystemSet::PassLife));
    }
}
// 标记哪些类型的上下文需要世界矩阵的逆矩阵
// 目前， 设置TransfromWillchange、ClipPath的节点， 需要世界矩阵的逆矩阵计算
#[derive(Debug, Default)]
pub struct WorldInvertMark(pub bitvec::prelude::BitArray<[u32; 1]>, pub bitvec::prelude::BitArray<[u32; 5]> );

// 标记哪些Pass2d需要计算世界矩阵的逆矩阵
pub fn mark_world_invert<T: Send + Sync, const STYLE_INDEX: usize>(
    mark_type: OrInitSingleRes<RenderContextMarkType<T>>,
    mut invert_mark: OrInitSingleResMut<WorldInvertMark>,
) {
    invert_mark.0.set(***mark_type, true);
    invert_mark.1.set(STYLE_INDEX, true);
}

// 计算世界矩阵的逆矩阵
pub fn calc_world_invert(
    context_mark_changed: ComponentChanged<RenderContextMark>,
    global_dirty: SingleRes<GlobalDirtyMark>,
    mut query: Query<(Ticker<&WorldMatrix>, &mut WorldMatrixInvert, &RenderContextMark)>,
    invert_mark: OrInitSingleRes<WorldInvertMark>,
) {
    let martix_dirty = match global_dirty.mark.get(OtherDirtyType::WorldMatrix as usize) {
        Some(r) => *r,
        None => false,
    };
    // 矩阵不脏, 需要计算世界矩阵逆矩阵的属性也不脏， 则直接跳过
    if !martix_dirty && (context_mark_changed.len() == 0 || !global_dirty.mark.contains(&invert_mark.1))  {
        return;
    }

    // 计算逆矩阵
    for (world_matrix, mut world_matrix_invert, context_mark) in query.iter_mut() {
        if !((**context_mark).clone() & &invert_mark.0).any() {
            // 不需要计算世界矩阵的逆矩阵，修改is_valid未false，跳过
            if !world_matrix_invert.is_valid {
                world_matrix_invert.is_valid = false;
            }
            continue;
        }

        if !world_matrix_invert.is_valid {
            world_matrix_invert.is_valid = true;
        } else {
            if !world_matrix.is_changed(){
                // 逆矩阵有效， 并且矩阵未改变， 则不需要重新计算逆矩阵
                continue;
            }
        }
        world_matrix_invert.value = match world_matrix.0.try_inverse() {
            Some(r) => Some(WorldMatrix(r, world_matrix.1)),
            None => None,
        };
    }
}
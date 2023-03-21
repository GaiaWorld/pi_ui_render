/// 框架内部计算属性(Node)
pub mod calc;
/// 用户声明属性(Node)
pub mod user;
// DrawObject属性
pub mod draw_obj;
pub mod pass_2d;
mod root;

use bevy::ecs::{bundle::Bundle, prelude::Entity};
use pi_bevy_ecs_extend::prelude::{Down, Layer, Up};

use self::{
    calc::{DrawInfo, DrawList, EntityKey, IsShow, NodeState, View, RenderContextMark, TransformWillChangeMatrix},
    draw_obj::{BoxType, ClearColorBindGroup, CopyFboToScreen, PipelineMeta},
    pass_2d::{ChildrenPass, GraphId, ParentPassId, PostProcessList, ViewMatrix},
    root::{RenderTarget, RootDirtyRect}, user::{ClearColor, Overflow},
};

#[derive(Debug, Bundle, Default)]
pub struct NodeBundle {
    pub style_mark: calc::StyleMark,
    pub size: user::Size,
    pub matrix: calc::WorldMatrix,
    pub z_range: calc::ZRange,
    pub content_box: calc::ContentBox,
    pub layout: calc::LayoutResult,
    pub quad: calc::Quad,
    pub in_pass_id: calc::InPassId,
    pub down: Down,
    pub up: Up,
    pub layer: Layer,
    pub reder_context_mark: RenderContextMark,
    pub draw_list: DrawList,
    pub node_state: NodeState,
    pub is_show: IsShow,
}

#[derive(Bundle)]
pub struct DrawBundle {
    pub node_id: calc::NodeId,
    pub draw_state: draw_obj::DrawState,
    pub box_type: BoxType,
    // pub fs_defines: FSDefines,
    // pub vs_defines: VSDefines,
    pub pipeline_meta: PipelineMeta,
    pub draw_info: DrawInfo,
}


#[derive(Bundle, Default)]
pub struct PassBundle {
    // pub node_id: calc::NodeId,
    pub parent_id: ParentPassId,
    pub camera: pass_2d::Camera,
    pub view_matrix: ViewMatrix,
    pub overflow_aabb: View,
    pub draw_list: pass_2d::Draw2DList,
    pub dirty_rect: pass_2d::DirtyRect,
    pub last_dirty_rect: pass_2d::LastDirtyRect,
    pub post_list: PostProcessList,
    pub children: ChildrenPass,
    pub graph_id: GraphId,
    pub will_change_matrix: TransformWillChangeMatrix,
}

impl PassBundle {
    pub fn new(parent_id: Entity) -> Self {
        Self {
            // node_id: NodeId(EntityKey(node_id)),
            parent_id: ParentPassId(EntityKey(parent_id)),
            view_matrix: Default::default(),
            overflow_aabb: Default::default(),
            camera: Default::default(),
            draw_list: Default::default(),
            dirty_rect: Default::default(),
            last_dirty_rect: Default::default(),
            post_list: Default::default(),
            children: Default::default(),
            will_change_matrix: Default::default(),
            graph_id: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct RootBundle {
    pub render_target: RenderTarget,
    pub copy_draw_obj: CopyFboToScreen,
    pub clear_color_group: ClearColorBindGroup,
    pub dirty_rect: RootDirtyRect,
	pub clear_color: ClearColor,
	pub overflow: Overflow,
}

impl Default for RootBundle {
    fn default() -> Self {
        Self { 
			render_target: Default::default(), 
			copy_draw_obj: Default::default(), 
			clear_color_group: Default::default(), 
			dirty_rect: Default::default(), 
			clear_color: Default::default(), 
			overflow: Overflow(true) 
		}
    }
}

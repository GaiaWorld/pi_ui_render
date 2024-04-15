//! Component、Bundle定义

pub mod calc;
pub mod draw_obj;
pub mod pass_2d;
pub mod root;
pub mod user;

use bevy_ecs::{
   	bundle::Bundle,
	prelude::{FromWorld, Entity},
};
use pi_bevy_ecs_extend::prelude::{Down, Layer, Up};

use self::{
    calc::{DrawInfo, DrawList, EntityKey, IsShow, NodeState, RenderContextMark, TransformWillChangeMatrix, View},
    draw_obj::{BoxType, CopyFboToScreen, FboInfo, InstanceIndex, PipelineMeta},
    pass_2d::{ChildrenPass, GraphId, ParentPassId, PostProcess, PostProcessInfo, RenderTarget},
    root::RootDirtyRect,
    user::Overflow,
};

/// 节点Bundle
#[derive(Debug, Bundle, Default, Clone, Serialize, Deserialize)]
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

/// 绘制对象Bundle
#[derive(Bundle)]
pub struct DrawBundle<T: FromWorld + Bundle> {
    pub node_id: calc::NodeId,
    pub draw_state: draw_obj::DrawState,
    pub box_type: BoxType,
    // pub fs_defines: FSDefines,
    // pub vs_defines: VSDefines,
    pub pipeline_meta: PipelineMeta,
    pub draw_info: DrawInfo,
    pub other: T,
}

/// 绘制对象Bundle（新）
#[derive(Bundle)]
pub struct DrawBundleNew<T: FromWorld + Bundle> {
    pub node_id: calc::NodeId,
	pub instance_index: InstanceIndex,
    // pub draw_state: draw_obj::DrawState,
    // pub box_type: BoxType,
    // pub fs_defines: FSDefines,
    // pub vs_defines: VSDefines,
    // pub pipeline_meta: PipelineMeta,
    pub draw_info: DrawInfo,
    pub other: T,
}

// impl<T: FromWorld + Bundle> FromWorld for DrawBundle<T> {
//     fn from_world(world: &mut bevy_ecs::prelude::World) -> Self {
// 		world.init_resource::<ProgramMetaRes<crate::shader::color::ProgramMeta>>();
// 		world.init_resource::<ShaderInfoCache>();
// 		world.init_resource::<PosVertexLayout>();


// 		let program_meta = world.get_resource::<ProgramMetaRes<crate::shader::color::ProgramMeta>>().unwrap();
// 		let cache = world.get_resource::<ShaderInfoCache>().unwrap();
// 		let vert_layout = world.get_resource::<PosVertexLayout>().unwrap();
//         DrawBundle {
// 			node_id: calc::NodeId(EntityKey::null()),
//             draw_state: Default::default(),
//             box_type: Default::default(),
//             pipeline_meta: PipelineMeta {
//                 program: (*program_meta).clone(),
//                 state: cache.common.clone(),
//                 vert_layout: (**vert_layout).clone(),
//                 defines: Default::default(),
//             },
//             draw_info: Default::default(),
// 			other: T::from_world(world),
// 		}
//     }
// }


/// 通道Bundle
/// 注意这个类型与unity中Pass的区别，Gui中，Pass并不一定会对应一个渲染目标
/// 一些属性，意图在节点以及其所有递归子节点上生效，常常需要成为一个Pass
/// 比如，Opacity属性，需要将自身及其所有递归子节点整体，作半透明处理，有Opacity组件的节点会成为一个Pass，将设置此Bundle包含的所有组件
/// 比如，TranformWillChange，将自身及其所有的子节点整体进行变化，有TranformWillChange组件的节点会成为一个Pass，将设置此Bundle包含的所有组件
/// Opacity与TranformWillChange不同的是，
/// Opacity需要将自身及其所有子节点渲染到一个fbo上，再将该fbo渲染到屏幕时，做半透明处理
/// TranformWillChange不需要中间Fbo的过程，直接改变每个节点的投影视图矩阵即可
/// 在gui中，想Opacity这种，需要额外fbo渲的属性，会被加入到PostProcessList中
/// 每个Pass自身包含的渲染对象及其所有递归子节点的渲染对象（如果某个子节点也是一个Pass，该几点及其递归子节点都不包含在内），都在帧推时，被收集在Draw2DList中
/// 由于Opacity，TranformWillChange在节点树中会处于不确定的层，因此，这些Pass节点也组织为一颗树的结构
/// PostProcessList被加入了像Opacity这样的效果时，该Pass在渲染图中有一个对应的节点，
/// PostProcessList中，没有任何效果时，Pass中Draw2DList的渲染依赖于父Pass，当父Pass渲染时，也会递归渲染该Pass中的Draw2DList，如果父Pass中也没有任何效果，则该Pass和其父Pass依赖于Pass的Pass渲染，以此类推（根节点尽管可能不存在任何效果，但一定会对应一个渲染图节点）
#[derive(Bundle, Default)]
pub struct PassBundle {
    // pub node_id: calc::NodeId,
    pub parent_id: ParentPassId,
    pub camera: pass_2d::Camera,
    pub overflow_aabb: View,
    pub draw_list: pass_2d::Draw2DList,
    pub post_list: PostProcess,
    pub dirty_rect: pass_2d::DirtyRect,
    pub dirty_mark: pass_2d::DirtyMark,
    pub post_list_info: PostProcessInfo,
    pub children: ChildrenPass,
    pub graph_id: GraphId,
    pub will_change_matrix: TransformWillChangeMatrix,
    pub render_target: RenderTarget,
	pub instance_index: InstanceIndex,
	pub fbo_info: FboInfo,
}

impl PassBundle {
    pub fn new(parent_id: Entity) -> Self {
        Self {
            // node_id: NodeId(EntityKey(node_id)),
            parent_id: ParentPassId(EntityKey(parent_id)),
            overflow_aabb: Default::default(),
            camera: Default::default(),
            draw_list: Default::default(),
            dirty_rect: Default::default(),
            dirty_mark: Default::default(),
            post_list: Default::default(),
            post_list_info: Default::default(),
            children: Default::default(),
            will_change_matrix: Default::default(),
            graph_id: Default::default(),
            render_target: Default::default(),
			instance_index: InstanceIndex::default(),
			fbo_info: FboInfo::default(),
        }
    }
}

/// 根节点Bundle（如果是根节点，会有该Bundle包含的所有组件）
#[derive(Bundle)]
pub struct RootBundle {
    pub render_target: RenderTarget,
    pub copy_draw_obj: CopyFboToScreen,
    // pub clear_color_group: ClearColorBindGroup,
    pub dirty_rect: RootDirtyRect,
    // pub clear_color: ClearColor,
    pub overflow: Overflow,
    // pub root_instance: RootInstance,
}

impl Default for RootBundle {
    fn default() -> Self {
        Self {
            render_target: Default::default(),
            copy_draw_obj: Default::default(),
            // clear_color_group: Default::default(),
            dirty_rect: Default::default(),
            // clear_color: Default::default(),
            overflow: Overflow(true),
            // root_instance: RootInstance::default(),
        }
    }
}

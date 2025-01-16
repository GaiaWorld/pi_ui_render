//! Component、Bundle定义

pub mod calc;
pub mod draw_obj;
pub mod pass_2d;
pub mod root;
pub mod user;

// use calc::IsDisplay;
use pi_world::{prelude::{Bundle, Entity, FromWorld}, world::{ComponentIndex, World}};
use pi_bevy_ecs_extend::prelude::{Down, Layer, Up};
use user::SvgInnerContent;

use self::{
    calc::{DrawInfo, DrawList, EntityKey, IsShow, NodeState, RenderContextMark, TransformWillChangeMatrix, View},
    draw_obj::{BoxType, FboInfo, InstanceIndex},
    pass_2d::{ChildrenPass, GraphId, ParentPassId, PostProcess, PostProcessInfo, RenderTarget},
    root::RootDirtyRect,
    user::{ClassName, Overflow},
};
use pi_bevy_render_plugin::asimage_url::RenderTarget as RenderTarget1;


#[derive(Clone)]
pub struct SettingComponentIds {
    pub down: ComponentIndex,
    pub up: ComponentIndex,
    pub layer: ComponentIndex,
    pub node_state: ComponentIndex,
    pub size: ComponentIndex,
    pub margin: ComponentIndex,
    pub padding: ComponentIndex,
    pub border: ComponentIndex,
    pub position: ComponentIndex,
    pub min_max: ComponentIndex,
    pub flex_container: ComponentIndex,
    pub flex_normal: ComponentIndex,
    pub z_index: ComponentIndex,
    pub overflow: ComponentIndex,
    pub opacity: ComponentIndex,
    pub blend_mode: ComponentIndex,
    pub show: ComponentIndex,
    pub transform: ComponentIndex,
    pub background_color: ComponentIndex,
    pub border_color: ComponentIndex,
    pub background_image: ComponentIndex,
    pub background_image_texture: ComponentIndex,
    pub background_image_clip: ComponentIndex,
    pub mask_image: ComponentIndex,
    pub mask_image_clip: ComponentIndex,
    pub hsi: ComponentIndex,
    pub blur: ComponentIndex,
    pub clip_path: ComponentIndex,
    pub background_image_mod: ComponentIndex,
    pub border_image: ComponentIndex,
    pub border_image_texture: ComponentIndex,
    pub border_image_clip: ComponentIndex,
    pub border_image_slice: ComponentIndex,
    pub border_image_repeat: ComponentIndex,
    pub border_radius: ComponentIndex,
    pub sdf_slice: ComponentIndex,
    pub sdf_uv: ComponentIndex,
    pub box_shadow: ComponentIndex,
    pub text_style: ComponentIndex,
    pub text_shadow: ComponentIndex,
    pub text_outer_glow: ComponentIndex,
    pub transform_will_change: ComponentIndex,
    pub text_content: ComponentIndex,
    pub animation: ComponentIndex,
    pub transition: ComponentIndex,
    pub class_name: ComponentIndex,
    pub as_image: ComponentIndex,
    pub text_overflow: ComponentIndex,

    pub style_mark: ComponentIndex,
    pub matrix: ComponentIndex,
    pub z_range: ComponentIndex,
    pub content_box: ComponentIndex,
    pub layout: ComponentIndex,
    pub quad: ComponentIndex,
    pub in_pass_id: ComponentIndex,
    pub render_context_mark: ComponentIndex,
    pub draw_list: ComponentIndex,
    pub is_show: ComponentIndex,
    // pub is_display: ComponentIndex,

    pub svg: ComponentIndex,
}

impl FromWorld for SettingComponentIds {
    fn from_world(world: &mut World) -> Self {
        Self {
            down: world.init_component::<Down>(),
            up: world.init_component::<Up>(),
            layer: world.init_component::<Layer>(),
            node_state: world.init_component::<NodeState>(),
            size: world.init_component::<self::user::Size>(),
            margin: world.init_component::<self::user::Margin>(),
            padding: world.init_component::<self::user::Padding>(),
            border: world.init_component::<self::user::Border>(),
            position: world.init_component::<self::user::Position>(),
            min_max: world.init_component::<self::user::MinMax>(),
            flex_container: world.init_component::<self::user::FlexContainer>(),
            flex_normal: world.init_component::<self::user::FlexNormal>(),
            z_index: world.init_component::<self::user::ZIndex>(),
            overflow: world.init_component::<self::user::Overflow>(),
            opacity: world.init_component::<self::user::Opacity>(),
            blend_mode: world.init_component::<self::user::BlendMode>(),
            show: world.init_component::<self::user::Show>(),
            transform: world.init_component::<self::user::Transform>(),
            background_color: world.init_component::<self::user::BackgroundColor>(),
            border_color: world.init_component::<self::user::BorderColor>(),
            background_image: world.init_component::<self::user::BackgroundImage>(),
            background_image_clip: world.init_component::<self::user::BackgroundImageClip>(),
            mask_image: world.init_component::<self::user::MaskImage>(),
            mask_image_clip: world.init_component::<self::user::MaskImageClip>(),
            hsi: world.init_component::<self::user::Hsi>(),
            blur: world.init_component::<self::user::Blur>(),
            clip_path: world.init_component::<self::user::ClipPath>(),
            background_image_mod: world.init_component::<self::user::BackgroundImageMod>(),
            border_image: world.init_component::<self::user::BorderImage>(),
            border_image_clip: world.init_component::<self::user::BorderImageClip>(),
            border_image_slice: world.init_component::<self::user::BorderImageSlice>(),
            border_image_repeat: world.init_component::<self::user::BorderImageRepeat>(),
            border_radius: world.init_component::<self::user::BorderRadius>(),
            sdf_slice: world.init_component::<self::calc::SdfSlice>(),
            sdf_uv: world.init_component::<self::calc::SdfUv>(),
            box_shadow: world.init_component::<self::user::BoxShadow>(),
            text_style: world.init_component::<self::user::TextStyle>(),
            text_shadow: world.init_component::<self::user::TextShadow>(),
            text_outer_glow: world.init_component::<self::user::TextOuterGlow>(),
            transform_will_change: world.init_component::<self::user::TransformWillChange>(),
            text_content: world.init_component::<self::user::TextContent>(),
            animation: world.init_component::<self::user::Animation>(),
            transition: world.init_component::<self::user::Transition>(),
            class_name: world.init_component::<self::user::ClassName>(),
            as_image: world.init_component::<self::user::AsImage>(),
            text_overflow: world.init_component::<self::user::TextOverflowData>(),

            background_image_texture: world.init_component::<self::calc::BackgroundImageTexture>(),
            border_image_texture: world.init_component::<self::calc::BorderImageTexture>(),

            style_mark: world.init_component::<self::calc::StyleMark>(),
            matrix: world.init_component::<self::calc::WorldMatrix>(),
            z_range: world.init_component::<self::calc::ZRange>(),
            content_box: world.init_component::<self::calc::ContentBox>(),
            layout: world.init_component::<self::calc::LayoutResult>(),
            quad: world.init_component::<self::calc::Quad>(),
            in_pass_id: world.init_component::<self::calc::InPassId>(),
            render_context_mark: world.init_component::<self::calc::RenderContextMark>(),
            draw_list: world.init_component::<self::calc::DrawList>(),
            is_show: world.init_component::<self::calc::IsShow>(),
            // is_display: world.init_component::<self::calc::IsDisplay>(),

            svg: world.init_component::<SvgInnerContent>(),
        }
    }
}

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
    // pub is_display: IsDisplay,
    pub class_name: ClassName,
}

// /// 绘制对象Bundle
// #[derive(Bundle)]
// pub struct DrawBundle<T: FromWorld + Bundle + 'static> {
//     pub node_id: calc::NodeId,
//     pub draw_state: draw_obj::DrawState,
//     pub box_type: BoxType,
//     // pub fs_defines: FSDefines,
//     // pub vs_defines: VSDefines,
//     pub pipeline_meta: PipelineMeta,
//     pub draw_info: DrawInfo,
//     pub other: T,
// }

/// 绘制对象Bundle（新）
#[derive(Bundle)]
pub struct DrawBundleNew<T: FromWorld + Bundle + 'static> {
    pub node_id: calc::NodeId,
	pub instance_index: InstanceIndex,
    // pub draw_state: draw_obj::DrawState,
    pub box_type: BoxType,
    // pub fs_defines: FSDefines,
    // pub vs_defines: VSDefines,
    // pub pipeline_meta: PipelineMeta,
    pub draw_info: DrawInfo,
    pub other: T,
}

// impl<T: FromWorld + Bundle> FromWorld for DrawBundle<T> {
//     fn from_world(world: &mut pi_world::prelude::World) -> Self {
// 		world.init_single_res::<ProgramMetaRes<crate::shader::color::ProgramMeta>>();
// 		world.init_single_res::<ShaderInfoCache>();
// 		world.init_single_res::<PosVertexLayout>();


// 		let program_meta = world.get_single_res::<ProgramMetaRes<crate::shader::color::ProgramMeta>>().unwrap();
// 		let cache = world.get_single_res::<ShaderInfoCache>().unwrap();
// 		let vert_layout = world.get_single_res::<PosVertexLayout>().unwrap();
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
    // pub mark: Pass2DMark,
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
    pub render_target1: RenderTarget1,
}

impl PassBundle {
    pub fn new(parent_id: Entity) -> Self {
        Self {
            // node_id: NodeId(EntityKey(node_id)),
            parent_id: ParentPassId(EntityKey(parent_id)),
            ..Default::default()
        }
    }
}

/// 根节点Bundle（如果是根节点，会有该Bundle包含的所有组件）
#[derive(Bundle)]
pub struct RootBundle {
    pub render_target: RenderTarget,
    pub dirty_rect: RootDirtyRect,
    pub overflow: Overflow,
}

impl Default for RootBundle {
    fn default() -> Self {
        Self {
            render_target: Default::default(),
            // clear_color_group: Default::default(),
            dirty_rect: Default::default(),
            // clear_color: Default::default(),
            overflow: Overflow(true),
            // root_instance: RootInstance::default(),
        }
    }
}

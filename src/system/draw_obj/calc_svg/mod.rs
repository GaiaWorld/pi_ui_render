use bevy_app::{Plugin, Update};
use bevy_window::AddFrameEvent;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::FrameDataPrepare;

use crate::{
    components::{
        draw_obj::SvgMark,
        user::{SvgContent, SvgInnerContent},
    },
    resource::SvgRenderObjType,
    system::system_set::UiSystemSet,
};

use self::calc_tex::{calc_sdf2_text, text_svg, update_sdf2_texture};

use super::life_drawobj::{draw_object_life_new, update_render_instance_data};

pub mod calc_tex;
use bevy_ecs::{schedule::IntoSystemConfigs, query::Changed};

pub const SVG_ORDER: u8 = 8;
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        println!("add SvgPlugin");
        app
            .add_systems(Update, text_svg)
            .add_frame_event::<ComponentEvent<Changed<SvgInnerContent>>>()
            // 创建drawobj
            .add_systems(
                Update,
                draw_object_life_new::<SvgInnerContent, SvgRenderObjType, SvgMark, { SVG_ORDER }>.in_set(UiSystemSet::LifeDrawObject).after(text_svg),
            )
            // 统计drawobj的实例长度
            // .add_systems(
            // 	Update,
            // 	calc_sdf2_text_len
            // 		.in_set(FrameDataPrepare)
            // 		.after(UiSystemSet::LifeDrawObjectFlush)
            // 		.before(update_render_instance_data)
            // 		.after(calc_layout)
            // )
            // 更新实例数据
            .add_systems(Update, calc_sdf2_text.in_set(UiSystemSet::PrepareDrawObj))
            // 更新纹理
            .add_systems(
                Update,
                update_sdf2_texture
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .after(text_svg)
                    .before(calc_sdf2_text),
            )
            ;
    }
}

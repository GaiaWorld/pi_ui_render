mod text;
mod text_glyph;
mod text_shadow;
mod text_split;
mod text_texture;

use bevy_ecs::prelude::{Changed, IntoSystemConfigs, Resource};
use bevy_app::{Plugin, Update, App};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_render::font::{Size, FontSheet};

use crate::{
    components::{calc::NodeState, draw_obj::TextMark, user::TextContent},
    resource::{draw_obj::PosUv1VertexLayout, ShareFontSheet, TextRenderObjType},
    system::{
        node::{layout::calc_layout, world_matrix::cal_matrix},
        system_set::UiSystemSet,
    },
};
use bevy_window::AddFrameEvent;

use self::{text::calc_text, text_shadow::UiTextShadowPlugin, text_texture::calc_text_texture};

use super::{life_drawobj, set_world_marix::set_matrix_group};

#[derive(Debug, Resource, Default)]
pub struct IsRun(pub bool);

pub struct UiTextPlugin {
	pub use_sdf: bool
}

impl Plugin for UiTextPlugin {
    fn build(&self, app: &mut App) {
		let font_sheet = ShareFontSheet::new(&mut app.world, self.use_sdf);
        app.insert_resource(font_sheet)
            .add_frame_event::<ComponentEvent<Changed<NodeState>>>()
            .add_frame_event::<ComponentEvent<Changed<TextContent>>>()
            // 文字劈分
            .add_systems(Update, text_split::text_split.before(calc_layout).in_set(UiSystemSet::Layout))
            // 字形计算
            .add_systems(Update, text_glyph::text_glyph.after(cal_matrix).before(calc_text).in_set(UiSystemSet::Matrix))
            // 设置文字的的顶点、索引，和颜色、边框颜色、边框宽度的Uniform
            .add_systems(Update, 
                calc_text
                    .in_set(UiSystemSet::PrepareDrawObj)
                    .before(set_matrix_group)
                    .after(calc_text_texture)
                    .before(super::blend_mode::calc_drawobj_blendstate),
            )
            // 更新文字纹理
            .add_systems(Update, calc_text_texture.in_set(UiSystemSet::PrepareDrawObj))
            // 文字阴影
            .add_plugins(UiTextShadowPlugin);
		if !self.use_sdf {
			// 创建文字DrawObj
			app.add_systems(Update, 
				life_drawobj::draw_object_life::<
					TextContent,
					TextRenderObjType,
					TextMark,
					PosUv1VertexLayout,
					crate::shader::text::ProgramMeta,
					{ TEXT_ORDER },
				>
					.in_set(UiSystemSet::LifeDrawObject),
			);
		} else {
			// 创建文字DrawObj
			app.add_systems(Update, 
				life_drawobj::draw_object_life::<
					TextContent,
					TextRenderObjType,
					TextMark,
					PosUv1VertexLayout,
					crate::shader::text_sdf::ProgramMeta,
					{ TEXT_ORDER },
				>
					.in_set(UiSystemSet::LifeDrawObject),
			);
		}
		
    }
}

pub const TEXT_ORDER: u8 = 8;

#[derive(Debug, Default)]
pub struct TextureState {
    pub width: usize,
    pub height: usize,
    pub version: usize,
}

impl TextureState {
    // 返回（宽高是否发生改变，版本是否发生改变）
    pub fn is_change(&mut self, size: &Size<usize>, font_sheet: &FontSheet) -> (bool, bool) {
        let size_is_change = size.width != self.width || size.height != self.height;
        if size_is_change {
            self.width = size.width;
            self.height = size.height;
        }
		let use_sdf = font_sheet.font_mgr().use_sdf();
       
		let version = if use_sdf {
			font_sheet.sdf_texture_version()
		} else {
			font_sheet.texture_version()
		};

		let version_is_change = version != self.version;
		if version_is_change {
			self.version = version;
		}

        (size_is_change, version_is_change)
    }
}

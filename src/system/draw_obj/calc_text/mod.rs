mod text_glyph;
mod text_split;
mod text;
mod text_shadow;
mod text_texture;

use bevy::prelude::{Plugin, Changed, IntoSystemConfig};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_render::font::Size;

use crate::{resource::{ShareFontSheet, TextRenderObjType, draw_obj::PosUvColorVertexLayout}, components::{calc::NodeState, user::TextContent, draw_obj::TextMark}, system::{system_set::UiSystemSet, node::{layout::calc_layout, world_matrix::cal_matrix}, AddEvent}};

use self::{text::calc_text, text_texture::calc_text_texture, text_shadow::UiTextShadowPlugin};

use super::{life_drawobj, set_world_marix::set_matrix_group};

pub struct UiTextPlugin;

impl Plugin for UiTextPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
			.init_resource::<ShareFontSheet>()
			.add_frame_event::<ComponentEvent<Changed<NodeState>>>()
			.add_frame_event::<ComponentEvent<Changed<TextContent>>>()
			// 文字劈分
			.add_system(text_split::text_split.before(calc_layout).in_set(UiSystemSet::Layout))
			// 字形计算
			.add_system(
				text_glyph::text_glyph
					.after(cal_matrix)
					.before(calc_text)
					.in_set(UiSystemSet::Matrix),
			)
			// 创建文字DrawObj
			.add_system(
				life_drawobj::draw_object_life::<
					TextContent,
					TextRenderObjType,
					TextMark,
					PosUvColorVertexLayout,
					crate::shader::text::ProgramMeta,
					{ TEXT_ORDER },
				>
					.in_set(UiSystemSet::LifeDrawObject),
			)
			// 设置文字的的顶点、索引，和颜色、边框颜色、边框宽度的Uniform
			.add_system(calc_text.in_set(UiSystemSet::PrepareDrawObj).before(set_matrix_group).after(calc_text_texture).before(super::blend_mode::calc_drawobj_blendstate))
			// 更新文字纹理
			.add_system(calc_text_texture.in_set(UiSystemSet::PrepareDrawObj))
			// 文字阴影
			.add_plugin(UiTextShadowPlugin)
		;
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
	pub fn is_change(&mut self, size: &Size<usize>, version: usize) -> (bool, bool) {
	
		let size_is_change = size.width != self.width || size.height != self.height;
		if size_is_change {
			self.width = size.width;
			self.height = size.height;
		}

		let version_is_change = version != self.version;
		if version_is_change {
			self.version = version;
		}

		(size_is_change, version_is_change)
	}
}
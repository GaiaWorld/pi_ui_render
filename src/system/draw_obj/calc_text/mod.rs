mod text_glyph;
mod text_split;
mod text;
mod text_shadow;
mod text_texture;

use pi_render::font::Size;
pub use text::UiTextPlugin;

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
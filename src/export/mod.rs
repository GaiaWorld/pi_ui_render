
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub mod json_parse;
pub mod native_index;

pub mod style;
// pub mod layout;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct Gui(pub crate::gui::Gui);

#[cfg(not(target_arch = "wasm32"))]
pub struct Gui(pub crate::gui::Gui);






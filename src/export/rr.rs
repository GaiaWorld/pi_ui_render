// pub use super::Engine as Gui;

// /**
//  * 获取canvas资源
//  */
// pub fn get_canvas_source(
//     commands: &mut UserCommands,
//     soruce: u32, // 是否缓存
// ) -> i32 {
//     -1
// }

// /**
//  * canvas宽高改变时调用(分配纹理成功，返回对应索引，否则返回-1)
//  * @return __jsObj 纹理
// */
// pub fn set_canvas_size(
//     commands: &mut UserCommands,
//     node: f64,
//     width: u32,
//     height: u32,
//     soruce: u32, // 是否缓存
//     need_depth: bool, // 是否需要深度缓冲区
//                  // avail_width: u32,
//                  // avail_height: u32,
// ) -> i32 {
//     -1
// }

// #[allow(unused_attributes)]
// pub fn get_canvas_target(commands: &mut UserCommands, index: usize) -> Option<usize> {
//     None
// }

// #[allow(unused_attributes)]
// pub fn get_canvas_rect(commands: &mut UserCommands, index: usize) -> JsValue {

//     let mut dyn_atlas_set = gui.gui.dyn_atlas_set.lend_mut();
//     let dyn_atlas_set = dyn_atlas_set.borrow_mut();
//     let rect = dyn_atlas_set.get_rect(index).unwrap();

//     JsValue::from_serde(&CanvasRect(
//         rect.mins.x as u32,
//         rect.mins.y as u32,
//         (rect.maxs.x - rect.mins.x) as u32,
//         (rect.maxs.y - rect.mins.y) as u32,
//     ))
//     .unwrap()
// }

// /**
//  * canvas内容发生改变时，应该调用此方法更新gui渲染
// */
// pub fn update_canvas(commands: &mut UserCommands, _node: u32) {
// }



// #[macro_use()]
// macro_rules! set_attr {
//     ($world:ident, $node_id:ident, $name:ident, $name1:ident, $name2: expr, $value:expr, $key: ident) => {
//         let node_id = $node_id as usize;
//         let world = unsafe { &mut *($world as usize as *mut GuiWorld) };
//         let attr = world.gui.$key.lend_mut();
//         let value = $value;
//         $crate::paste::item! {
//             let r = &mut attr[node_id];
//             r.$name.$name1 = value;
//             attr.get_notify_ref().modify_event(node_id, $name2, 0);
//         }
//     };
// }
use crate::{
    components::calc::DrawList,
    resource::{fragment::DebugInfo, ShareFontSheet},
};
use pi_world::{prelude::Plugin, schedule::Last};
use pi_world::{
    query::Query,
    single_res::{SingleRes, SingleResMut},
};

pub fn sys_debug_info(
    mut debug_info: SingleResMut<DebugInfo>,
    font_sheet: SingleRes<ShareFontSheet>,
    query: Query<(&DrawList,)>,
) {
    let mut size = 0;
    for i in font_sheet.0.borrow().font_mgr().table.sdf2_table.fonts.values() {
        size += i.debug_size()
    }

    debug_info.font_size = size;

    size = 0;
    query.iter().for_each(|v| size += v.0.len());

    debug_info.draw_obj_count = size;
}

/// 使用sdf2的方式渲染文字
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
        let info = DebugInfo::default();
        app.world.insert_single_res(info);
        app.add_system(Last, sys_debug_info);
    }
}

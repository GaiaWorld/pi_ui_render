
use std::{collections::HashSet, mem::transmute};
use pi_bevy_ecs_extend::prelude::{Down, EntityTag, Layer, Up};
use pi_bevy_render_plugin::{_request_document};
use pi_null::Null;
use pi_world::{
    prelude::{App, Entity, IntoSystemConfigs, Plugin}, 
    world::World, event::ComponentRemoved, filter::{Changed, With}, 
    query::Query, schedule::{End}, system_params::{Local}
};
use pi_ws::{connect::WsSocket, utils::WsFrameType};
use pi_tcp::{
    connect::TcpSocket,
};
use json::JsonValue;
use crate::{components::{user::{Size}}, tools::{_request_computed, _request_global_info, _request_global_interface, _request_right_key_element, _request_showbox, _request_style, init_show_box_node, init_showbox_pipeline}};
use super::{init_node, node_info};

use pi_bevy_render_plugin::spector::{send_cmd, sys_parse_cmd, CMDCalls, Cmd, SpectorNode, CMDS, SOCKETS};

pub struct PluginSpectorUI;
impl Plugin for PluginSpectorUI {
    fn build(&self, app: &mut App) {
        init_showbox_pipeline(&mut app.world);
        app.add_startup_system(End, init_show_box_node);
        app.add_system(End, tool_run1.after(sys_parse_cmd));

        let cmdscalls = app.world.get_single_res_mut::<CMDCalls>().unwrap();
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_COMPUTED),         cmd_request_computed);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_DOCUMENT),         cmd_request_document);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_GLOBAL_INFO),      cmd_request_global_info);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_GLOBAL_INTERFACE), cmd_request_global_interface);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_MODIFY_STYLE),     cmd_request_modify_style);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_RIGHTKEY_ELE),     cmd_request_right_key_element);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_SHOWBOX),          cmd_request_showbox);
        cmdscalls.cmdcalls.insert(String::from(CMD_REQUEST_STYLE),            cmd_request_style);
    }
}

pub fn tool_run1(
    query: Query<(Entity, &Layer, &Up), (With<Size>, Changed<Layer>)>,
    query2: Query<(Entity, &Layer, &Up), With<Size>>,
    query1: Query<(&Down, &Up, Option<&EntityTag>)>,
    delete: ComponentRemoved<Size>,
    mut local: Local<HashSet<Entity>>,
) {
    let sockes = SOCKETS.get().unwrap();
    let sockes = sockes.lock().unwrap();

    if sockes.len() == 0 {
        return; // 没有连接的客户端，直接返回
    }

    // 删除节点
    for e in delete.iter() {
        if local.remove(e) {
            let del_node_cmd = Cmd {
                cmd: "remove-child".to_string(),
                payload: RemoveNode {
                    // parentUniqueID: unsafe {  transmute::<_, f64>(up.parent()) },
                    // parentUniqueIDString: format!("{:?}", up.parent()),
                    uniqueID: unsafe {  transmute::<_, f64>(e) },
                    // uniqueIDString: format!("{:?}", e),
                }
            };
            send_cmd(del_node_cmd, &*sockes);
        }
    }


    // 删除或添加节点
    for (entity, layer, _up) in query.iter() {
        let mut e = entity;
        loop {
            let (entity, layer, up) = query2.get(e).unwrap();
            if layer.layer().is_null() {
                local.remove(&entity);
            } else if local.contains(&entity) {
                break;
            }
            if !up.parent().is_null() {
                e = up.parent();
            } else {
                break;
            }
        }

        if e != entity {
            let (_entity, _layer, mut up) = query2.get(e).unwrap();
            if layer.layer().is_null() {
                // 删除节点
                let del_node_cmd = Cmd {
                    cmd: "remove-child".to_string(),
                    payload: RemoveNode {
                        // parentUniqueID: unsafe {  transmute::<_, f64>(up.parent()) },
                        // parentUniqueIDString: format!("{:?}", up.parent()),
                        uniqueID: unsafe {  transmute::<_, f64>(e) },
                        // uniqueIDString: format!("{:?}", e),
                    }
                };
                send_cmd(del_node_cmd, &*sockes);
            } else {
                // 添加节点
                let mut index = 0;
                while !up.prev().is_null() {
                    index += 1;
                    let (mut _entity, _layer, up1) = query2.get(up.prev()).unwrap();
                    up = up1;
                }
                let mut guinode = SpectorNode::default();
                init_node(e, &mut guinode, &query1);
                let add_node_cmd = Cmd {
                    cmd: "add-child".to_string(),
                    payload: AddNode {
                        parentUniqueID: unsafe {  transmute::<_, f64>(up.parent()) },
                        parentUniqueIDString: format!("{:?}", up.parent()),
                        child: guinode,
                        index: index,
                    }
                };
                send_cmd(add_node_cmd, &*sockes);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
struct AddNode {
    parentUniqueID: f64,
    parentUniqueIDString: String,
    child: SpectorNode,
    index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
struct RemoveNode {
    uniqueID: f64,
}

pub const CMD_REQUEST_DOCUMENT: &'static str            = "request-document";
pub const CMD_REQUEST_STYLE: &'static str               = "request-style";
pub const CMD_REQUEST_COMPUTED: &'static str            = "request-computed";
pub const CMD_REQUEST_SHOWBOX: &'static str             = "request-showbox";
pub const CMD_REQUEST_RIGHTKEY_ELE: &'static str        = "request-right-key-element";
pub const CMD_REQUEST_GLOBAL_INTERFACE: &'static str    = "request-global-interface";
pub const CMD_REQUEST_GLOBAL_INFO: &'static str         = "request-global-info";
pub const CMD_REQUEST_MODIFY_STYLE: &'static str        = "request-modify-style";

/// cmd `request-document`
fn cmd_request_document(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    _request_document(world).drain(..).for_each(|cmd| {
        let msg = serde_json::to_string(&cmd).unwrap();
        if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
            log::error!("send error: {}", e);
        }
    });
}
/// cmd `request-style`
fn cmd_request_style(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let _select_node_id: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };
            let data = _request_style(world, _select_node_id);
            let data = data.as_bytes().to_vec();
            if let Err(e) = connect.send(WsFrameType::Text, data) {
                log::error!("send error: {}", e);
            }
        },
        r => log::error!("cmd invalid: {:?}", r),
    };
}


/// cmd `request-computed`
fn cmd_request_computed(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let select_node_id: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };
            let msg = _request_computed(world, select_node_id);
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        r => log::error!("cmd invalid: {:?}", r),
    };
}
/// cmd `request-showbox`
fn cmd_request_showbox(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let select_node_id: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };
            _request_showbox(world, select_node_id);
        },
        r => return log::error!("cmd invalid: {:?}", r),
    };
}
/// cmd `request-right-key-element`
fn cmd_request_right_key_element(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    let x = obj["x"].as_f32().unwrap();
    let y = obj["y"].as_f32().unwrap();

    let msg = _request_right_key_element(world, x, y);
    if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
        log::error!("send error: {}", e);
    }
}
/// cmd `request-global-interface`
fn cmd_request_global_interface(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    let msg = _request_global_interface(world);
    println!("========= request-global-interface msg: {}", msg);
    if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
        log::error!("send error: {}", e);
    }
}
/// cmd `request-global-info`
fn cmd_request_global_info(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    
    match obj.get("payload") {
        Some(JsonValue::Short(request_cmd)) => {
            let msg = _request_global_info(world, request_cmd.as_str());
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        }
        _ => {

        }
    }
}
/// cmd `request-modify-style`
fn cmd_request_modify_style(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    
}

pub fn request_right_key_element( x: f32, y: f32) {
    let sockes = SOCKETS.get().unwrap();
    let sockes = sockes.lock().unwrap();
    let mut values =  sockes.values();
    
    if values.len() > 0 {
        let socket = values.next().unwrap().clone();
        CMDS.get().unwrap().push(( format!("{{\"cmd\": \"request-right-key-element\", \"x\": {}, \"y\": {}}}", x, y), socket));
    }
}

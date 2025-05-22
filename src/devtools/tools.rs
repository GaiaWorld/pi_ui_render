
use std::{collections::HashSet, mem::transmute, sync::{Arc, Mutex, OnceLock}};
use ahash::HashMap;
use crossbeam::queue::SegQueue;
use pi_async_rt::rt::serial::AsyncRuntimeBuilder;
use pi_bevy_ecs_extend::prelude::{Down, Layer, Up};
use pi_null::Null;
use pi_world::{app::App, event::ComponentRemoved, filter::{Changed, With}, query::Query, schedule::End, schedule_config::IntoSystemConfigs, system_params::Local, world::{Entity, World}};
use std::io::Result;
use pi_ws::{connect::WsSocket, server::WebsocketListener, utils::{ChildProtocol, WsFrameType, WsSession}};
use futures::future::{FutureExt, LocalBoxFuture};
use pi_tcp::{SocketConfig, SocketEvent,
        connect::TcpSocket,
        server::{PortsAdapterFactory, SocketListener}};
use json::JsonValue;
use crate::{components::user::{ClassName, Size}, devtools::{get_document_tree, get_roots}};

use super::{init_node, node_info, GuiNode};

static CMDS: OnceLock<Arc<SegQueue<(String, WsSocket<TcpSocket>)>>> = OnceLock::new();
static SOCKETS: OnceLock<Mutex<HashMap<usize, WsSocket<TcpSocket>>>> = OnceLock::new();

pub fn start_server(app: &mut App) {
    // 初始化全局变量
    CMDS.get_or_init(|| {
        Arc::new(SegQueue::new())
    });
    SOCKETS.get_or_init(|| {
        Mutex::new(HashMap::default())
    });

    // 启动一个http服务
    let _out = std::process::Command::new("node")
    .args([
        "src/devtools/http_server.js",
    ])
    .spawn();

    //启动一个websocket服务
    start_websocket_server();

    app.add_system(End, tool_run);
    app.add_system(End, tool_run1.after(tool_run));
}

pub fn tool_run(world: &mut World) {
    parse_cmd1(world);
}
pub fn tool_run1(
    query: Query<(Entity, &Layer, &Up), (With<Size>, Changed<Layer>)>,
    query2: Query<(Entity, &Layer, &Up), With<Size>>,
    query1: Query<(&Down, &Up, Option<&ClassName>), With<Size>>,
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
                let mut guinode = GuiNode::default();
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
    child: GuiNode,
    index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
struct RemoveNode {
    uniqueID: f64,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
struct Cmd<T: serde::Serialize> {
    cmd: String,
    payload: T,
}

fn send_cmd<T: serde::Serialize>(cmd: Cmd<T>, sockets: &HashMap<usize, WsSocket<TcpSocket>>) {
    let cmd_string = serde_json::to_string(&cmd).unwrap();
    for socket in sockets.values() {
        if let Err(e) = socket.send(WsFrameType::Text, cmd_string.as_bytes().to_vec()) {
            log::error!("Error sending message: {:?}", e);
        }
    }
}

fn start_websocket_server() {

    let rt0 = AsyncRuntimeBuilder::default_local_thread(None, None);
    let rt1 = AsyncRuntimeBuilder::default_local_thread(None, None);

    let mut factory = PortsAdapterFactory::<TcpSocket>::new();
    factory.bind(3001,
                 Box::new(WebsocketListener::with_protocol(Arc::new(MyChildProtocol))));
    let mut config = SocketConfig::new("0.0.0.0", factory.ports().as_slice());
    config.set_option(16384, 16384, 16384, 16);

    match SocketListener::bind(vec![rt0, rt1],
                               factory,
                               config,
                               1024,
                               1024 * 1024,
                               1024,
                               16,
                               4096,
                               4096,
                               Some(1000)) {
        Err(e) => {
            println!("!!!> Websocket Listener Bind Error, reason: {:?}", e);
        },
        Ok(_driver) => {
            println!("===> Websocket Listener in: {:?}", "0.0.0.0:3001");
        }
    }

}


struct MyChildProtocol;

impl ChildProtocol<TcpSocket> for MyChildProtocol {
    fn protocol_name(&self) -> &str {
        "echo"
    }

    fn is_strict(&self) -> bool {
        false
    }

    fn decode_protocol(&self,
                       connect: WsSocket<TcpSocket>,
                       context: &mut WsSession) -> LocalBoxFuture<'static, Result<()>> {
        let uid = connect.get_uid();
        let sockes = SOCKETS.get().unwrap();
        let mut sockes = sockes.lock().unwrap();
        if sockes.get(&uid).is_none() {
            sockes.insert(uid, connect.clone());
        }

        let msg = context.pop_msg();
        // println!("!!!!!!receive ok, msg: {:?}", String::from_utf8(msg.clone()));
        CMDS.get().unwrap().push(( String::from_utf8(msg).unwrap(), connect));
        async move {
            Ok(())
        }.boxed_local()
    }

    fn close_protocol(&self,
                      connect: WsSocket<TcpSocket>,
                      _context: WsSession,
                      reason: Result<()>) -> LocalBoxFuture<'static, ()> {
        let uid = connect.get_uid();
        let sockes = SOCKETS.get().unwrap();
        sockes.lock().unwrap().remove(&uid);
        async move {
            if let Err(e) = reason {
                return println!("websocket closed, reason: {:?}", e);
            }

            println!("websocket closed");
        }.boxed_local()
    }

    fn protocol_timeout(&self,
                        _connect: WsSocket<TcpSocket>,
                        _context: &mut WsSession,
                        _event: SocketEvent) -> LocalBoxFuture<'static, Result<()>> {
        async move {
            println!("websocket timeout");

            Ok(())
        }.boxed_local()
    }
}
pub fn parse_cmd1(world: &mut World) {
    let cmds = CMDS.get().unwrap();

    let mut cur_cmd = cmds.pop();
    while let Some(cmd) = cur_cmd {
        if let Err(e) = parse_cmd(&cmd.0, cmd.1, world) {
            log::error!("parse cmd error: {}", e);
        }
        cur_cmd = cmds.pop();
    }


}

fn parse_cmd(cmd: &str, connect: WsSocket<TcpSocket>, world: &mut World) -> std::result::Result<(), String> {
    let parsed = json::parse(cmd);
    let obj = match parsed {
        Ok(JsonValue::Object(obj)) => obj,
        r => return Err(format!("message invalid: {:?}", r))
    };
    let cmd = match obj.get("cmd") {
        Some(JsonValue::Short(cmd)) => cmd,
        r => return Err(format!("cmd invalid: {:?}", r))
    };

    match cmd.as_str() {
        "request-document" => {
            let roots = get_roots(world);
            log::error!("root======{:?}", &roots);
            for root in roots.into_iter() {
                let msg = get_document_tree(world, root);
                let cmd = Cmd {
                    cmd: "document-data".to_string(),
                    payload: msg,
                };
                let msg = serde_json::to_string(&cmd).unwrap();
                if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                    log::error!("send error: {}", e);
                }
            }
            
        },
        "request-style" => {
            let _select_node_id: Entity = match obj.get("payload") {
                Some(JsonValue::Number(select_node_id)) => unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) },
                r => return Err(format!("cmd invalid: {:?}", r))
            };
        },
        "request-computed" => {
            let select_node_id: Entity = match obj.get("payload") {
                Some(JsonValue::Number(select_node_id)) => unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) },
                r => return Err(format!("cmd invalid: {:?}", r))
            };
            let msg = node_info(world, select_node_id);
            let cmd = Cmd {
                cmd: "computed-data".to_string(),
                payload: msg,
            };
            let msg = serde_json::to_string(&cmd).unwrap();
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        "request-showbox" => {},
        "request-right-key-element" => {},
        "request-render-graph" => {},
        "request-system-graph" => {},
        "request-modify-style" => {},

        r => return Err(format!("cmd invalid: {:?}", r)) 
    };
    Ok(())
}

// /**
//  * 定义公共的数据结构
//  */
// pub enum Cmd {
// 	AddChild = "add-child", // 添加节点
// 	RemoveChild = "remove-child", // 移除节点
// 	updateChild = "update-child", // 添加节点
// 	DocumentData = "document-data", // document的json数据
// 	InitDocument = "init-document", // document初始化成功时，需要发送的指令
// 	StyleData = "style-data", // Style数据
// 	ComputedData = "computed-data",
// 	RenderGraphData = "render-graph-data", // 渲染图数据
// 	SystemGraphData = "system-graph-data", // 系统图数据
// 	RightKeyElement = "right-key-element", // 右键元素id

// 	ShowGuiDevpanel = "show-gui-devpanel",

// 	RequestAll = "request-document", // devpanel被打开，请求显示document
// 	RequestStyle = "request-style", // 请求style数据
// 	RequestComputed = "request-computed", // 请求计算数据
// 	RequestShowbox = "request-showbox", // 请求显示包围盒
// 	RequestRightKeyElement = "request-right-key-element", // 请求右键元素id
// 	RequestRenderGraph = "request-render-graph", // 请求渲染图
// 	RequestSystemGraph = "request-system-graph", // 请求系统图

// 	RequestModifyStyle = "request-modify-style", // 请求修改元素的style

// 	ShowDocument = "show-document", // 显示的document

	
// }


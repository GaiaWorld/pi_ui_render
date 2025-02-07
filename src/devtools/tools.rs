
use std::{collections::HashSet, mem::transmute, sync::{Arc, OnceLock}};
use ahash::HashMap;
use crossbeam::queue::SegQueue;
use pi_async_rt::rt::serial::AsyncRuntimeBuilder;
use pi_bevy_ecs_extend::prelude::{Down, Layer, Up};
use pi_null::Null;
use pi_world::{app::App, filter::{Changed, With}, query::Query, schedule::Last, world::{Entity, World}};
use std::io::Result;
use pi_ws::{connect::WsSocket, server::WebsocketListener, utils::{ChildProtocol, WsFrameType, WsSession}};
use futures::future::{FutureExt, LocalBoxFuture};
use pi_tcp::{SocketConfig, SocketEvent,
        connect::TcpSocket,
        server::{PortsAdapterFactory, SocketListener}};
use json::JsonValue;
use crate::{components::user::{ClassName, Size}, devtools::get_document_tree};

use super::{init_node, node_info, GuiNode};

static CMDS: OnceLock<Arc<SegQueue<(String, WsSocket<TcpSocket>)>>> = OnceLock::new();
static SOCKETS: OnceLock<HashMap<usize, WsSocket<TcpSocket>>> = OnceLock::new();

pub fn start_server(app: &mut App) {
    // 初始化全局变量
    CMDS.get_or_init(|| {
        Arc::new(SegQueue::new())
    });
    SOCKETS.get_or_init(|| {
        HashMap::default()
    });

    // 启动一个http服务
    let _out = std::process::Command::new("node")
    .args([
        "src/devtools/http_server.js",
    ]);

    //启动一个websocket服务
    start_websocket_server();

    app.add_system(Last, tool_run);
}

pub fn tool_run(world: &mut World) {
    parse_cmd1(world);
}
pub fn tool_run1(
    query: Query<(Entity, &Layer, &Up), (With<Size>, Changed<Layer>)>,
    query1: Query<(&Down, &Up, Option<&ClassName>), With<Size>>,
    mut local: HashSet<Entity>,
) {
    for (entity, layer, up) in query.iter() {
        let mut e = entity;
        loop {
            let (entity, layer, up) = query.get(e).unwrap();
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
            if layer.layer().is_null() {
                // 删除节点
            } else {
                // 添加节点
                let (_entity, _layer, mut up) = query.get(e).unwrap();
                let mut index = 0;
                while !up.prev().is_null() {
                    index += 1;
                    let (mut _entity, _layer, up1) = query.get(up.prev()).unwrap();
                    up = up1;
                }
                let mut guinode = GuiNode::default();
                init_node(e, &mut guinode, &query1);
                let add_node_cmd = AddNode {
                    parentUniqueID: unsafe {  transmute::<_, f64>(up.parent()) },
                    parentUniqueIDString: format!("{:?}", up.parent()),
                    child: guinode,
                    index: index,
                };

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

#[derive(Serialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
struct Cmd<T: serde::Serialize> {
    cmd: String,
    payload: T,
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
            println!("===> Websocket Listener Bind Ok");
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
        let msg = context.pop_msg();
        println!("!!!!!!receive ok, msg: {:?}", String::from_utf8(msg.clone()));
        CMDS.get().unwrap().push(( String::from_utf8(msg).unwrap(), connect));
        async move {
            // if let Some(hibernate) = connect.hibernate(Ready::Writable) {
            //     let connect_copy = connect.clone();
            //     thread::spawn(move || {
            //         thread::sleep(Duration::from_millis(1000));
            //         while !connect_copy.wakeup(Ok(())) {
            //             //唤醒被阻塞，则休眠指定时间后继续尝试唤醒
            //             thread::sleep(Duration::from_millis(15));
            //         }
            //     });
            //     let start = Instant::now();
            //     if let Err(e) = hibernate.await {
            //         //唤醒后返回错误，则立即返回错误原因
            //         return Err(e);
            //     }
            //     println!("!!!!!!wakeup hibernate ok, time: {:?}", start.elapsed());
            // }

            // for _ in 0..1 {
            //     if let Err(e) = connect.send(msg_type.clone(), msg.clone()) {
            //         return Err(e);
            //     }
            // }

            // println!("reply msg ok");
            Ok(())
        }.boxed_local()
    }

    fn close_protocol(&self,
                      _connect: WsSocket<TcpSocket>,
                      _context: WsSession,
                      reason: Result<()>) -> LocalBoxFuture<'static, ()> {
        // let uid = SOCKETS.get()
        // SOCKETS
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
        Some(JsonValue::String(cmd)) => cmd,
        r => return Err(format!("cmd invalid: {:?}", r))
    };

    match cmd.as_str() {
        "request-document" => {
            let msg = get_document_tree(world);
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
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


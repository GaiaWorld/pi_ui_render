
use std::{collections::HashSet, mem::transmute, sync::{Arc, Mutex, OnceLock}};
use ahash::HashMap;
use crossbeam::queue::SegQueue;
use pi_async_rt::rt::serial::AsyncRuntimeBuilder;
use pi_bevy_ecs_extend::prelude::{Down, Layer, OrInitSingleRes, OrInitSingleResMut, Up};
use pi_bevy_render_plugin::{node::Node, NodeId, PiRenderDevice, PiRenderGraph, PiScreenTexture, RenderContext};
use pi_hal::font::sdf_gpu::create_indices;
use pi_null::Null;
use pi_share::ShareRefCell;
use pi_spatial::quad_helper::intersects;
use pi_style::style::Aabb2;
use pi_world::{app::App, event::ComponentRemoved, filter::{Changed, With}, query::Query, schedule::{End, Last}, schedule_config::IntoSystemConfigs, single_res::{SingleRes, SingleResMut}, system_params::{Local, SystemParam}, world::{ComponentIndex, Entity, World}};
use serde_json::json;
use std::io::Result;
use pi_ws::{connect::WsSocket, server::WebsocketListener, utils::{ChildProtocol, WsFrameType, WsSession}};
use futures::future::{BoxFuture, FutureExt, LocalBoxFuture};
use pi_tcp::{SocketConfig, SocketEvent,
        connect::TcpSocket,
        server::{PortsAdapterFactory, SocketListener}};
use json::JsonValue;
use crate::{components::{calc::{EntityKey, InPassId, IsRotate, IsShow, Quad, ZRange}, pass_2d::ParentPassId, root::{RootScale, Viewport}, user::{ClassName, Overflow, Point2, Size}}, devtools::{get_document_tree, get_global_info, get_roots}, resource::{draw_obj::{create_common_pipeline_state, LastGraphNode}, IsRun, QuadTree}};
use crate::devtools::{get_style, get_class_names, get_class};
use super::{init_node, node_info, GuiNode};
use wgpu::util::DeviceExt;
use wgpu::CommandEncoder;
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
    // let _out = std::process::Command::new("node")
    // .args([
    //     "src/devtools/http_server.js",
    // ])
    // .spawn();

    //启动一个websocket服务
    start_websocket_server();
    init_showbox_pipeline(&mut app.world);
    app.add_startup_system(End, init_show_box_node);
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
        // println!("!!!!!!receive ok, msg: {:?}", (String::from_utf8(msg.clone()), uid));
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
        println!("websocket closed");
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
            log::error!("parse cmd error111111111: {}", e);
        }
        cur_cmd = cmds.pop();
    }


}

fn parse_cmd(cmd: &str, connect: WsSocket<TcpSocket>, world: &mut World) -> std::result::Result<(), String> {
    // println!("=========== parse_cmd: {}", cmd);
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
            let style = get_style(world, _select_node_id);

      
            
            let c = style.split(";").collect::<Vec<&str>>();
            let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            for  i in 0..c.len() - 1 {
                let arr = c[i].split(":").collect::<Vec<&str>>();
                style[arr[0]] = arr[1].into();
            }

            let class_name = get_class_names(world, _select_node_id);
            let class_name = serde_json::from_str::<serde_json::Value>(&class_name).unwrap();
            println!("======= class_name: {:?}", class_name);
            let mut classs = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            for class_name in class_name.as_array().unwrap() {
                let class_name = class_name.as_u64().unwrap() as u32;
                let mut r = serde_json::from_str::<serde_json::Value>("{}").unwrap();
                let c = get_class(&world, class_name);
               
                   let c = c.split(";");
                    for a in c {
                        let arr = a.split(":").collect::<Vec<&str>>();
                        println!("========= arr: {:?}", (&a, &arr));
                        if arr.len() > 1 {
                            r[arr[0]] = arr[1].into();
                        }
                    }
                    classs[class_name.to_string()] = r;
                
            }
            let msg = format!("{{\"cmd\": \"style-data\", \"payload\": {{\"style\": {}, \"classs\": {} }} }}", style.to_string(), classs);
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
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
        "request-showbox" => {
            let select_node_id: Entity = match obj.get("payload") {
                Some(JsonValue::Number(select_node_id)) => unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) },
                r => return Err(format!("cmd invalid: {:?}", r))
            };

            let info = world.get_single_res_mut::<ShowboxInfo>().unwrap();
            if info.id != select_node_id{
                info.id = select_node_id;
            }

        },
        "request-right-key-element" => {
            let x = obj["x"].as_f32().unwrap();
            let y = obj["y"].as_f32().unwrap();
    
            if let Some((id, root_id)) = lookup_ele_by_pointer(world, x, y){
                let msg = format!("{{\"cmd\": \"right-key-element\" , \"payload\": {{\"uniqueID\": {}, \"documentUniqueID\": {} }}}}", id, root_id);
                println!("========= msg: {}", msg);
                if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                    log::error!("send error: {}", e);
                }
            }
        },
        "request-global-interface" => {
            let msg = "{\"cmd\": \"global-info-interface\" , \"payload\": [[\"ExecutionGraph\",\"graph\"],[\"ToopGraph\",\"graph\"],[\"GlobalInfo\",\"json\"]]}";
            println!("========= request-global-interface msg: {}", msg);
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        "request-global-info" => {
            let request_cmd = match obj.get("payload") {
                Some(JsonValue::Short(request_cmd)) => request_cmd,
                r => return Err(format!("cmd invalid: {:?}", r))
            };
            let info = match request_cmd.as_str() {
                "ExecutionGraph" => {
                    let g = world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
                    g.dump_graphviz()
                },
                "ToopGraph" => {
                    let g = world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
                    g.dump_toop_graphviz()
                },
                "GlobalInfo" => {
                    let info = get_global_info(&world);
                    serde_json::to_string(&info).unwrap()
                },
                _ => "".to_string(),
            };
            // let info = "digraph Render {\"\"}";
            // let j = serde_json::from_str::<serde_json::Value>(&info).unwrap();
            // println!("============ request-global-info msg: {:?}", j);
            let msg = format!("{{\"cmd\": \"global-info-data\", \"payload\": {{\"name\":\"{}\", \"data\": {:?}}}}}", request_cmd, info);
            println!("============ request-global-info msg: {}", msg);
            // let j = serde_json::from_str::<serde_json::Value>(&msg).unwrap();
            
            // println!("============ request-global-info msg: {}", j);
            
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        "request-modify-style" => {},

        r => return Err(format!("cmd invalid: {:?}", r)) 
    };
    Ok(())
}

// pub struct 

pub fn request_right_key_element( x: f32, y: f32) {
    let sockes = SOCKETS.get().unwrap();
    let sockes = sockes.lock().unwrap();
    let mut values =  sockes.values();
    
    if values.len() > 0 {
        let socket = values.next().unwrap().clone();
        CMDS.get().unwrap().push(( format!("{{\"cmd\": \"request-right-key-element\", \"x\": {}, \"y\": {}}}", x, y), socket));
    }
}

// 初始化渲染图的根节点
pub fn init_show_box_node(
    last_graph_id: OrInitSingleResMut<LastGraphNode>,
    mut rg: SingleResMut<PiRenderGraph>,
	r: OrInitSingleRes<IsRun>
) {
    if r.0 {
		return;
	}
    
    match rg.add_node("show_box".to_string(), ShowBoxNode, Null::null(), Null::null()) {
        Ok(r) => {
            rg.add_depend(last_graph_id.0, r).unwrap();
            let _ = rg.set_finish(r, true);
        },
        Err(e) => log::error!("node: {:?}, {:?}", "show_box".to_string(), e),
    };
    // rg
}

pub struct ShowBoxNode;

impl Node for ShowBoxNode {
    type RunParam = (SingleRes<'static, PiScreenTexture>, SingleResMut<'static, ShowboxInfo>, Query<'static, &'static Quad, ()> );
    type BuildParam = ();
    type ResetParam = ();
    // // 释放纹理占用
    // fn reset<'a>(&'a mut self) {
    //     // self.out_put_target = None;
    //     // self.target = None;
    // }

    /// 用于给pass2d分配fbo
    fn build<'a>(
        &'a mut self,
        // world: &'a mut pi_world::world::World,
        _param: &'a mut Self::BuildParam,
        _context: pi_bevy_render_plugin::RenderContext,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
    ) -> std::result::Result<(), String> {
        Ok(())
    }

    fn run<'a>(
        &'a mut self,
        param: &'a Self::RunParam,
        _context: RenderContext,
        mut commands: ShareRefCell<CommandEncoder>,
        _id: Entity,
        _from: &'a [Entity],
        _to: &'a [Entity],
    ) -> BoxFuture<'a, std::result::Result<(), String>> {
        // println!("=========== run");
        Box::pin(async move {
            if let Ok(quad) = param.2.get(param.1.id) {
                let mut rpass = commands.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("debug showbox"),
                        color_attachments: &[
                            Some(wgpu::RenderPassColorAttachment {
                                view: param.0.as_ref().unwrap().view().as_ref().unwrap(),
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                            })
                        ],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    }
                );
    
                rpass.set_pipeline(&param.1.pipeline);
                // println!("view: {:?}", (quad.mins.x , quad.mins.y, quad.maxs.x - quad.mins.x, quad.maxs.y - quad.mins.y,));
                rpass.set_viewport(quad.mins.x , quad.mins.y, quad.maxs.x - quad.mins.x, quad.maxs.y - quad.mins.y, 0.0, 1.0);
                // rpass.set_bind_group(0, &bind_group1, &[]);
                // rpass.set_bind_group(1, &bind_group2, &[]);
    
                rpass.set_index_buffer(param.1.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                rpass.set_vertex_buffer(0, param.1.vertex_buffer.slice(..));
    
                rpass.draw_indexed(0..6, 0, 0..1 as u32);
            }
            
            Ok(())
        })
    }
    
    fn reset<'a>(
        &'a mut self,
        param: &'a mut Self::ResetParam,
        context: RenderContext,
        id: Entity,
    ) {

    }
}
pub struct ShowboxInfo{
    pipeline: pi_render::rhi::pipeline::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    id: Entity,
}

fn init_showbox_pipeline(world: &mut World){
    let device = world.get_single_res::<PiRenderDevice>().unwrap();
    
    let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Glsl {
            shader: include_str!("./showbox.vert").into(),
            stage: naga::ShaderStage::Vertex,
            defines: Default::default(),
        },
    });

    // Load the shaders from disk
    let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Glsl {
            shader: include_str!("./showbox.frag").into(),
            stage: naga::ShaderStage::Fragment,
            defines: Default::default(),
        },
    });

    let vertexs = [
        -1.0f32, -1.0, 
         -1.0, 1.0, 
         1.0, -1.0,
         1.0, 1.0
    ]; // 获取网格数据
    println!("vertexs: {:?}", vertexs);


    // 创建网格数据
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&vertexs),
        usage: wgpu::BufferUsages::VERTEX,
    });


    let index_data = create_indices();
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&index_data),
        usage: wgpu::BufferUsages::INDEX,
    });

    let primitive = wgpu::PrimitiveState::default();

    // primitive.
    // let mut tt: ColorTargetState = swapchain_format.into();
    // tt.blend = Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING);
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let r = create_common_pipeline_state();
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vs,
            entry_point: Some("main"),
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                },
            ],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs,
            entry_point: Some("main"),
            targets: &r.targets,
            compilation_options: Default::default(),
        }),
        primitive,
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });
    world.insert_single_res(ShowboxInfo{
        pipeline: render_pipeline,
        vertex_buffer,
        index_buffer,
        id: Entity::null(),
        // bbox: Aabb2::new(Point2::new(0., 0.), Point2::new(0., 0.))
    });
}



pub struct AbQueryArgs<'a> {
    // query: Query<'s, 'w, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
    // query_parent: Query<'s, 'w, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    world: &'a World,
    aabb: Aabb2,
    result: EntityKey,
    root_id: Entity,
    max_z: usize,
}

fn lookup_ele_by_pointer(world: &mut World, x: f32, y: f32) -> Option<(f64, f64)>{

    let aabb = Aabb2::new(Point2::new(x, y), Point2::new(x, y));
    let mut args = AbQueryArgs {
        world: world,
        aabb,
        result: EntityKey::null(),
        root_id:Entity::null(),
        max_z: usize::MIN,
    };
    world.get_single_res::<QuadTree>().unwrap().query(&aabb, intersects, &mut args, ab_query_func);
    if args.result.is_null() {
        None
    } else {
        Some(unsafe {( transmute(args.result), transmute(args.root_id)) })
    }
   
}

/// aabb的ab查询函数, aabb的oct查询函数应该使用intersects
fn ab_query_func(arg: &mut AbQueryArgs, id: EntityKey, aabb: &Aabb2, _bind: &()) {
	// log::warn!("ab_query_func======={:?}", id);
    let (layer, z_range, inpass) = match (
        arg.world.get_component::<Layer>(*id), 
        arg.world.get_component::<IsShow>(*id), 
        arg.world.get_component::<ZRange>(*id), 
        arg.world.get_component::<InPassId>(*id),
    ) {
        // 如果enable false 表示不接收事件, visibility为false， 也无法接收事件、不在树上也不能接收事件
        (Ok(r0), Ok(r1), Ok(r2), Ok(r3)) if (r0.layer() != 0 && r1.get_enable() && r1.get_visibility() && r1.get_display()) => (r0, r2, r3),
        _ => return,
    };
    if intersects(&arg.aabb, aabb) {
        // 取最大z的node
        if z_range.start > arg.max_z {
            // 检查是否有裁剪，及是否在裁剪范围内
            let mut inpass = *(inpass.0);
            while !inpass.is_null() {
                // log::warn!("inpass======={:?}", (inpass, id));
                if let (Ok(parent), Ok(quad)) = (
                    arg.world.get_component::<ParentPassId>(inpass),
                    arg.world.get_component::<Quad>(inpass),
                ){
                    inpass = parent.0;
                    if let Ok(oveflow) = arg.world.get_component::<Overflow>(inpass) {
                        if oveflow.0 {
                            if !intersects(&arg.aabb, quad) {
                                return; // 如果不想交，直接返回，该点不能命中该节点
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            arg.root_id = layer.root();
            arg.result = id;
            arg.max_z = z_range.start;
        }
    }
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


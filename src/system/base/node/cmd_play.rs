//! 记录gui的指令

use std::{collections::VecDeque, mem::transmute};

use crate::{
    components::{
        calc::EntityKey, root::Viewport, user::{serialize::StyleTypeReader, style_attr_list_to_buffer, ClassName, StyleAttribute}
    },
    prelude::UserCommands,
    resource::{fragment::NodeTag, CmdType, FragmentCommand, NodeCommand}, system::base::node::user_setting,
};

use pi_style::style::Aabb2;
use pi_world::{insert::Insert, prelude::{App, Entity, IntoSystemConfigs, Plugin}};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::FrameState;
use pi_null::Null;
use pi_slotmap::SecondaryMap;

use crate::system::{system_set::UiSystemSet, RunState};
use crate::prelude::UiStage;

#[derive(Debug, Clone, Copy, Default)]
pub enum TraceOption {
    #[default]
    None,
    Record,
    Play,
}

pub struct UiCmdTracePlugin {
    pub option: TraceOption,
}

impl Plugin for UiCmdTracePlugin {
    fn build(&self, app: &mut App) {
        log::info!("self.option==============={:?}", self.option);
        match self.option {
            TraceOption::Record => {
                app.add_system(UiStage, cmd_record
                    .in_set(UiSystemSet::Setting)
                    .before(user_setting::user_setting1)
                );
            }
            TraceOption::Play => {
                app.add_system(UiStage, cmd_play
                    .in_set(UiSystemSet::Setting)
                    .before(user_setting::user_setting1)
                );
            }
            TraceOption::None => return,
        };
        app.world.init_single_res::<PlayState>();
        app.world.init_single_res::<Records>();
        app.world.init_single_res::<CmdNodeCreate>();
    }
}

// 记录指令
pub fn cmd_record(
    mut user_commands: OrInitSingleResMut<UserCommands>,
    mut node_creates: OrInitSingleResMut<CmdNodeCreate>,
    mut records: OrInitSingleResMut<Records>,
    run_state: OrInitSingleRes<RunState>,
	frame_state: OrInitSingleRes<FrameState>,
) {
    records.cur_frame_count += 1;
    let cur_frame_count = records.cur_frame_count;
    if **run_state != RunState::SETTING  {
		if let FrameState::UnActive = **frame_state {
			records.run_state.push((**run_state, cur_frame_count));
		}
    }

    let mut node_init_commands = Vec::new();
    let mut ss = Vec::with_capacity(user_commands.style_commands.commands.len());
    for s in user_commands.style_commands.commands.iter() {
        to_attr(s.0, s.1, s.2, &user_commands.style_commands.style_buffer, &mut ss);
        if let Some(tag) = s.3 {
            node_init_commands.push((s.0, tag));
        }
    }

    
    

    let frame_index = records.cur_frame_count;
    if ss.len() == 0
        && user_commands.node_commands.len() == 0
        && user_commands.fragment_commands.len() == 0
        && user_commands.class_commands.len() == 0
        && user_commands.other_commands_list.len() == 0
        && node_creates.0.len() == 0
    {
        // let last_index = records.list.len() - 1;
        // records.list[last_index].frame_index += 1;
    } else {
        records.list.push(Record {
            frame_index,
            state: **run_state,
            node_commands: user_commands.node_commands.clone(),
			node_init_commands: node_init_commands,
            fragment_commands: user_commands.fragment_commands.clone(),
            style_commands: ss,
            class_commands: user_commands.class_commands.clone(),
            other_commands_list: user_commands.other_commands_list.clone(),
            nodes_creates: node_creates.0.clone(),
        });
        user_commands.other_commands_list.clear();
        // log::info!("node_creates============{:?}", node_creates);
    }
    node_creates.0.clear();
}


pub fn cmd_play(
    records: OrInitSingleResMut<Records>, 
    mut play_state: OrInitSingleResMut<PlayState>, 
    mut user_commands: OrInitSingleResMut<UserCommands>,
    insert: Insert<()>,
) {
    if !play_state.is_running {
        // 清空指令列表
        // 播放时， 忽略外部设置的任何其他指令， 只使用记录的指令来播放
        **user_commands = UserCommands::default();
        return; // 已经播完， 不需要继续播放
    }

    // 已经播放到最后一个，设置当前播放状态
    if play_state.next_state_index >= records.list.len() {
        play_state.is_running = false;
        play_state.cur_frame_count = 0;
        // **records = Records::default();
        return;
    }

    // 慢速播放设置
    if play_state.await_frame_count > 0 {
        play_state.await_frame_count -= 1;
        return;
    } else {
        if play_state.speed < 1.0 && play_state.speed > 0.0 {
            play_state.await_frame_count = (1.0 / play_state.speed) as usize;
        }
    }

    play_state.cur_frame_count += 1;

    let r = &records.list[play_state.next_state_index];
    // 还需要继续播放一些空帧
    if r.frame_index < play_state.cur_frame_count {
        return;
    }
    // 先创建实体， 并建立映射关系
    for x in r.nodes_creates.clone().iter() {
        let id = insert.insert(());
        play_state.node_map.insert(EntityKey(x.clone()), id);
    }

    let r = &records.list[play_state.next_state_index];

    let mut cmds = UserCommands::default();

    // 需要重新映射Entity
    for i in r.node_commands.iter() {
        let cmd = match i {
            NodeCommand::AppendNode(a, b) => {
                let a = match play_state.get_node(a) {
                    Some(r) => r,
                    None => continue,
                };
                #[cfg(not(target_arch="wasm32"))]
                let v = unsafe { transmute::<_, f64>(b) };
                #[cfg(target_arch="wasm32")]
                let v = unsafe { transmute::<_, f32>(b) };

                let b = if EntityKey(*b).is_null() || v == 0.0 {
                    EntityKey::null().0
                } else {
                    match play_state.get_node(b) {
                        Some(r) => r,
                        None => continue,
                    }
                };

                NodeCommand::AppendNode(a, b)
            }
            NodeCommand::InsertBefore(a, b) => {
                let a = match play_state.get_node(a) {
                    Some(r) => r,
                    None => continue,
                };
                #[cfg(not(target_arch="wasm32"))]
                let v = unsafe { transmute::<_, f64>(b) };
                #[cfg(target_arch="wasm32")]
                let v = unsafe { transmute::<_, f32>(b) };

                let b = if EntityKey(*b).is_null() || v == 0.0 {
                    EntityKey::null().0
                } else {
                    match play_state.get_node(b) {
                        Some(r) => r,
                        None => continue,
                    }
                };
                NodeCommand::InsertBefore(a, b)
            }
            NodeCommand::RemoveNode(a) => match play_state.get_node(a) {
                Some(r) => NodeCommand::RemoveNode(r),
                None => continue,
            },
            NodeCommand::DestroyNode(a) => match play_state.get_node(a) {
                Some(r) => NodeCommand::DestroyNode(r),
                None => continue,
            },
        };
        cmds.node_commands.push(cmd);
    }


    for (n, class_name) in r.class_commands.iter() {
        let node = match play_state.get_node(n) {
            Some(r) => r,
            None => continue,
        };
        cmds.class_commands.push((node, class_name.clone()))
    }

    for other_command in r.other_commands_list.iter() {
        match other_command {
            CmdType::RuntimeAnimationBindCmd(r) => {
                let mut r = r.clone();
                let node = match play_state.get_node(&r.2) {
                    Some(r) => r,
                    None => continue,
                };
                r.2 = node;
                cmds.push_cmd(r);
            }
            CmdType::ComponentCmd(r) => {
                let mut r = r.clone();
                let node = match play_state.get_node(&r.1) {
                    Some(r) => r,
                    None => continue,
                };
                r.1 = node;
                cmds.push_cmd(r);
            }
            CmdType::NodeCmdViewport(r) => {
                let mut r = r.clone();
                let node = match play_state.get_node(&r.1) {
                    Some(r) => r,
                    None => continue,
                };
                r.1 = node;

                if let Some(view_port) = play_state.view_port {
                    r.0 = Viewport(view_port);
                }
                cmds.push_cmd(r);
            }
            CmdType::NodeCmdRenderTargetType(r) => {
                let mut r = r.clone();
                let node = match play_state.get_node(&r.1) {
                    Some(r) => r,
                    None => continue,
                };
                r.1 = node;
                cmds.push_cmd(r);
            }
            CmdType::NodeCmdRenderClearColor(r) => {
                // let mut r = r.clone();
                // let node = match play_state.get_node(&r.1) {
                //     Some(r) => r,
                //     None => continue,
                // };
                // r.1 = node;
                cmds.push_cmd(r.clone());
            }
            CmdType::NodeCmdRenderRenderDirty(r) => {
                let mut r = r.clone();
                let node = match play_state.get_node(&r.1) {
                    Some(r) => r,
                    None => continue,
                };
                r.1 = node;
                cmds.push_cmd(r);
            }
            CmdType::NodeCmdRenderNodeBundle(r) => {
                let mut r = r.clone();
                let node = match play_state.get_node(&r.1) {
                    Some(r) => r,
                    None => continue,
                };
                r.1 = node;
                cmds.push_cmd(r);
            }
            CmdType::ExtendCssCmd(r) => {
                cmds.push_cmd(r.clone());
            }
            CmdType::DefaultStyleCmd(r) => {
                cmds.push_cmd(r.clone());
            }
            CmdType::ExtendFragmentCmd(r) => {
                cmds.push_cmd(r.clone());
            }
            CmdType::SdfCfgCmd(r) => {
				cmds.push_cmd(r.clone());
			},
            CmdType::SdfDefaultCharCmd(r) => {
				cmds.push_cmd(r.clone());
			},
            CmdType::Sdf2CfgCmd(r) => {
				cmds.push_cmd(r.clone());
			},
            // CmdType::SvgStrokeCmd(r) => { 
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::StrokeDasharrayCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgShapeCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgStrokeWidthCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgShapeWidthCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgShapeHeightCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgShapeXCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgShapeYCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
            // CmdType::SvgColorCmd(r) => {
            //     let mut r = r.clone();
            //     let node = match play_state.get_node(&r.0) {
            //         Some(r) => r,
            //         None => continue,
            //     };
            //     r.0 = node;
            //     cmds.push_cmd(r);
            // },
        };
    }

    for fragment_commands in r.fragment_commands.iter() {
        let r: Vec<Entity> = fragment_commands
            .entitys
            .iter()
            .map(|r| {
                if let None = play_state.get_node(r) {
                    log::warn!("xxxxxxx=========={:?}", r);
                }
                play_state.get_node(r).unwrap()
            })
            .collect();
        cmds.fragment_commands.push(FragmentCommand {
            key: fragment_commands.key,
            entitys: r,
        });
    }

    for (entity, tag) in r.node_init_commands.iter() {
        // log::warn!("node_init_commands:{:?} {:?}", entity, tag);
        cmds.init_node(play_state.get_node(&entity).unwrap(), *tag);
    }

    for s in r.style_commands.iter() {
        let class_mate = style_attr_list_to_buffer(&mut cmds.style_commands.style_buffer, &mut s.values.clone(), s.values.len());
        // log::warn!("style_commands:{:?}", s);
        cmds.style_commands
            .commands
            .push((play_state.get_node(&s.entity).unwrap(), class_mate.start, class_mate.end, None));
    }
    // log::info!("style_commands============{:?}", &cmds.style_commands.commands);
    // log::info!("node_commands============{:?}", &cmds.node_commands);
    // log::info!("other_commands_list============{:?}", &r.other_commands_list);

    play_state.next_state_index += 1;
    **user_commands = cmds;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleCmd {
    pub entity: Entity,
    pub values: VecDeque<StyleAttribute>,
}


pub fn to_attr(node: Entity, start: usize, end: usize, style_buffer: &Vec<u8>, list: &mut Vec<StyleCmd>) {
    let mut style_reader = StyleTypeReader::new(style_buffer, start, end);
    let mut v = VecDeque::new();
    while let Some(attr) = style_reader.to_attr() {
        v.push_back(attr);
    }

    if v.len() > 0 {
        list.push(StyleCmd { entity: node, values: v});
    }
}

// pub fn all_style_list_to_buffer(style_buffer: &mut Vec<u8>, style_list: &mut VecDeque<Attribute>, mut count: usize) -> ClassMeta {
// }

//
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Record {
    pub frame_index: usize, // 所在帧位置
    pub state: RunState,
    pub node_commands: Vec<NodeCommand>,
	pub node_init_commands: Vec<(Entity, NodeTag)>,
    pub fragment_commands: Vec<FragmentCommand>,
    pub style_commands: Vec<StyleCmd>,
    pub class_commands: Vec<(Entity, ClassName)>,
    pub other_commands_list: Vec<CmdType>, // 是CommandQueue中元素的枚举形式，便于序列化
    pub nodes_creates: Vec<Entity>,        // 创建的节点
}

// impl Record {
// 	pub fn len(&self) -> usize {
// 		self.style_commands.len() + self.node_commands.len() + self.fragment_commands.len() + self.class_commands.len() + self.other_commands_list.len() + self.nodes_creates.len()
// 	}
// }

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Records {
    pub list: Vec<Record>,
    // 记录在每个状态下运行多少次
    pub run_state: Vec<(RunState, usize)>,
    pub cur_frame_count: usize,
}


impl Records {
    pub fn clear(&mut self) {
        self.list.clear();
        self.run_state.clear();
        self.cur_frame_count = 0;
    }

    pub fn len(&self) -> usize { self.list.len() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    cur: usize,
    frames: usize,
}

// 播放状态
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayState {
    pub next_state_index: usize, //
    // pub need_play_empty_count: usize,
    pub cur_frame_count: usize,

    // 节点对应表（由于某些原因， record中记录的Entity与实际的Entity不匹配）（比如录制的时候是在有spine渲染的情况下进行的，而播放得时候没有spine， 会造成gui创建的实体id不同）
    // 将指令描述中的Entity修改映射为当前创建的Entity
    pub node_map: SecondaryMap<EntityKey, Entity>,
    pub is_running: bool,

    pub next_reord_index: usize,

    // 播放速度
    pub speed: f32,

    // 等待帧数量（减速播放时需要用到）
    pub await_frame_count: usize,
    pub view_port: Option<Aabb2>,
}

impl PlayState {
    fn get_node(&self, entity: &Entity) -> Option<Entity> {
        match self.node_map.get(EntityKey(*entity)) {
            Some(r) => Some(*r),
            None => None,
        }
    }
}

// 记录状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordState {
    pub empty_count: usize, //
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CmdNodeCreate(pub Vec<Entity>);

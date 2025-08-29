//! 记录gui的指令

use std::{collections::VecDeque, mem::transmute};

use crate::{
    components::{
        root::Viewport, user::{serialize::StyleTypeReader, ClassName, StyleAttribute}
    },
    prelude::UserCommands,
    resource::{fragment::NodeTag, CmdType, FragmentCommand, NodeCommand}, system::base::node::user_setting,
};

use bitvec::array::BitArray;
use pi_atom::Atom;
use pi_hash::XHashMap;
use pi_style::{style::Aabb2, style_parse::style_to_buffer, style_type::{BackgroundImageType, BorderImageType, ClassMeta, MaskImageType}};
use pi_world::{insert::Insert, prelude::{App, Entity, IntoSystemConfigs, Plugin}, schedule::First, world::World};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::{asimage_url::entity_to_asimage_url, FrameState, KeyRecord, PlayState, Records, StageCMDTrace, TraceOption, RECORD_UI_COMMAND};
use pi_null::Null;
use pi_slotmap::{KeyData, SecondaryMap};


pub struct UiCmdTracePlugin;

impl Plugin for UiCmdTracePlugin {
    fn build(&self, app: &mut App) {
        let option = app.world.get_single_res::<PlayState>().unwrap();
        log::info!("self.option==============={:?}", option.option);
        match option.option {
            TraceOption::Record => {
                app.add_system(First, sys_ui_cmd_record
                    .in_set(StageCMDTrace::After)
                    .before(user_setting::user_setting1)
                );
            }
            TraceOption::Play => {
                app.add_system(First, sys_ui_cmd_play
                    .in_set(StageCMDTrace::After)
                    .before(user_setting::user_setting1)
                );
            }
            TraceOption::None => return,
        };
        let option = app.world.get_single_res_mut::<Records>().unwrap();
        option.palycalls.insert(RECORD_UI_COMMAND, cmd_play_call);
        app.world.insert_single_res(CmdNodeCreate::default());
    }
}

// 记录指令
pub fn sys_ui_cmd_record(
    mut user_commands: OrInitSingleResMut<UserCommands>,
    mut node_creates: OrInitSingleResMut<CmdNodeCreate>,
    mut records: OrInitSingleResMut<Records>,
) {

    let mut node_init_commands = Vec::new();
    let mut ss = Vec::with_capacity(user_commands.style_commands.commands.len());
    for s in user_commands.style_commands.commands.iter() {
        to_attr(s.0, s.1, s.2, &user_commands.style_commands.style_buffer, &mut ss);
        if let Some(tag) = s.3 {
            node_init_commands.push((s.0, tag));
        }
    }

    node_creates.0.iter().for_each(|entity| {
        records.record_create(*entity);
    });

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
        let bin = postcard::to_stdvec::<UIRecord>(&UIRecord {
            node_commands: user_commands.node_commands.clone(),
			node_init_commands: node_init_commands,
            fragment_commands: user_commands.fragment_commands.clone(),
            style_commands: ss,
            class_commands: user_commands.class_commands.clone(),
            other_commands_list: user_commands.other_commands_list.clone(),
        });
        user_commands.other_commands_list.clear();
        // log::info!("node_creates============{:?}", node_creates);

        match bin {
            Ok(bin) => {
                records.record(RECORD_UI_COMMAND, bin);
            },
            Err(_) => {},
        }
    }
    node_creates.0.clear();
}


pub fn sys_ui_cmd_play(
    mut play_state: OrInitSingleResMut<PlayState>, 
    mut user_commands: OrInitSingleResMut<UserCommands>,
) {
    match play_state.playresult {
        pi_bevy_render_plugin::EReplayResult::None => {},
        pi_bevy_render_plugin::EReplayResult::End => {
            **user_commands = UserCommands::default();
        },
        pi_bevy_render_plugin::EReplayResult::AwaitFrame => {},
        pi_bevy_render_plugin::EReplayResult::NullFrame => {},
        pi_bevy_render_plugin::EReplayResult::IsLast => {},
        pi_bevy_render_plugin::EReplayResult::Ok => {},
    }
}

pub fn cmd_play_call(world: &mut World, data: &Vec<u8>, replayentities: &XHashMap<Entity, Entity>) {
    match postcard::from_bytes::<UIRecord>(data) {
        Ok(record) => {
            let r: UIRecord = record;
            
            let mut cmds = UserCommands::default();
            for i in r.node_commands.iter() {
                apply_node_command(&mut cmds, i, replayentities);
            }
            for (n, class_name) in r.class_commands.iter() {
                apply_class_name(&mut cmds, (*n, class_name.clone()), replayentities);
            }
            for other_command in r.other_commands_list.iter() {
                apply_nother_cmd(&mut cmds, other_command, replayentities);
            }
            for fragment_commands in r.fragment_commands.iter() {
                apply_fragment_command(&mut cmds, fragment_commands, replayentities);
            }
            for (entity, tag) in r.node_init_commands.iter() {
                apply_init_command(&mut cmds, (*entity, tag.clone()), replayentities);
            }
            for s in r.style_commands.iter() {
                apply_style_command(&mut cmds, s, replayentities);
            }

            let user_commands = world.get_single_res_mut::<UserCommands>().unwrap();
            **user_commands = cmds;
        },
        Err(_) => {},
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleCmd {
    pub entity: Entity,
    pub values: VecDeque<StyleAttribute>,
}

pub fn parse_asimage(url: &str, play_state: &XHashMap<Entity, Entity>) -> String {
    if !url.starts_with("asimage:://") {
        return url.to_string();
    }

    let r = url["asimage:://".len()..].to_string();
    let mut r = r.split("v");
    let index: u32 = match r.next() {
        Some(r) => match r.parse() {
            Ok(r) => r,
            Err(_) => return "".to_string(),
        },
        None => return "".to_string(),
    };
    let version: u32 = match r.next() {
        Some(r) => match r.parse() {
            Ok(r) => r,
            Err(_) => return "".to_string(),
        },
        None => return "".to_string(),
    };
    let entity = Entity::from(KeyData::from_ffi((u64::from(version) << 32) | u64::from(index)));
    return match play_state.get(&entity) {
        Some(r) => entity_to_asimage_url(*r),
        None => "".to_string(),
    };
}

pub fn style_attr_list_to_buffer(play_state: &XHashMap<Entity, Entity>, style_buffer: &mut Vec<u8>, style_list: &mut VecDeque<StyleAttribute>, mut count: usize) -> ClassMeta {
    let start: usize = style_buffer.len();
    let mut class_meta = ClassMeta {
        start,
        end: start,
        class_style_mark: BitArray::default(),
    };

    loop {
        if count == 0 {
            break;
        }
        let r = style_list.pop_front().unwrap();
        match r {
            StyleAttribute::Reset(r) => {
                style_buffer.push((r & 255) as u8);
                style_buffer.push((r >> 8) as u8);
            }
            StyleAttribute::Set(r) => {
                let r = match r {
                    pi_style::style_parse::Attribute::BackgroundImage(r) => {
                        pi_style::style_parse::Attribute::BackgroundImage(BackgroundImageType( Atom::from(parse_asimage(r.0.as_str(), play_state)) ))
                    },
                    pi_style::style_parse::Attribute::BorderImage(r) => {
                        pi_style::style_parse::Attribute::BorderImage(BorderImageType( Atom::from(parse_asimage(r.0.as_str(), play_state)) ))
                    },
                    pi_style::style_parse::Attribute::MaskImage(r) => {
                        let r = match r.0 {
                            pi_style::style::MaskImage::Path(r) => pi_style::style::MaskImage::Path( Atom::from(parse_asimage(r.as_str(), play_state))),
                            r => r,
                        };
                        pi_style::style_parse::Attribute::MaskImage(MaskImageType(r))
                    },
                    // pi_style::style_parse::Attribute::AnimationName(_) => (),
                    // pi_style::style_parse::Attribute::AnimationDuration(_) => (),
                    // pi_style::style_parse::Attribute::AnimationTimingFunction(_) => (),
                    // pi_style::style_parse::Attribute::AnimationIterationCount(_) => (),
                    // pi_style::style_parse::Attribute::AnimationDirection(_) => (),
                    // pi_style::style_parse::Attribute::AnimationFillMode(_) => (),
                    // pi_style::style_parse::Attribute::AnimationPlayState(_) => (),
                    // pi_style::style_parse::Attribute::AnimationDelay(_) => (),
                    // pi_style::style_parse::Attribute::BackgroundImage(_) => (),
                    // pi_style::style_parse::Attribute::BackgroundImageClip(_) => (),
                    r => r
                };
                style_to_buffer(style_buffer, r, &mut class_meta);
                
            },
        }

        count -= 1;
    }
    class_meta.end = style_buffer.len();

    class_meta
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
pub struct UIRecord {
    pub node_commands: Vec<NodeCommand>,
	pub node_init_commands: Vec<(Entity, NodeTag)>,
    pub fragment_commands: Vec<FragmentCommand>,
    pub style_commands: Vec<StyleCmd>,
    pub class_commands: Vec<(Entity, ClassName)>,
    pub other_commands_list: Vec<CmdType>, // 是CommandQueue中元素的枚举形式，便于序列化
}

// impl Record {
// 	pub fn len(&self) -> usize {
// 		self.style_commands.len() + self.node_commands.len() + self.fragment_commands.len() + self.class_commands.len() + self.other_commands_list.len() + self.nodes_creates.len()
// 	}
// }

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct Records {
//     pub list: Vec<Record>,
//     // 记录在每个状态下运行多少次
//     pub run_state: Vec<(RunState, usize)>,
//     pub cur_frame_count: usize,
// }


// impl Records {
//     pub fn clear(&mut self) {
//         self.list.clear();
//         self.run_state.clear();
//         self.cur_frame_count = 0;
//     }

//     pub fn len(&self) -> usize { self.list.len() }
// }


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CmdNodeCreate(pub Vec<Entity>);

pub trait TReplayState {
    fn get(&self, v: &Entity) -> Option<Entity>;
    fn view_port(&self) -> Option<Aabb2>;
}

pub fn apply_node_command(cmds: &mut UserCommands, i: &NodeCommand, play_state: &XHashMap<Entity, Entity>) {
    let cmd = match i {
        NodeCommand::AppendNode(a, b) => {
            let a = match play_state.get(a) {
                Some(r) => *r,
                None => return,
            };
            #[cfg(not(target_arch="wasm32"))]
            let v = unsafe { transmute::<_, f64>(b) };
            #[cfg(target_arch="wasm32")]
            let v = unsafe { transmute::<_, f32>(b) };

            let b = if b.is_null() || v == 0.0 {
                Entity::null()
            } else {
                match play_state.get(b) {
                    Some(r) => *r,
                    None => return,
                }
            };

            NodeCommand::AppendNode(a, b)
        }
        NodeCommand::InsertBefore(a, b) => {
            let a = match play_state.get(a) {
                Some(r) => *r,
                None => return,
            };
            #[cfg(not(target_arch="wasm32"))]
            let v = unsafe { transmute::<_, f64>(b) };
            #[cfg(target_arch="wasm32")]
            let v = unsafe { transmute::<_, f32>(b) };

            let b = if b.is_null() || v == 0.0 {
                Entity::null()
            } else {
                match play_state.get(b) {
                    Some(r) => *r,
                    None => return,
                }
            };
            NodeCommand::InsertBefore(a, b)
        }
        NodeCommand::RemoveNode(a) => match play_state.get(a) {
            Some(r) => NodeCommand::RemoveNode(*r),
            None => return,
        },
        NodeCommand::DestroyNode(a) => match play_state.get(a) {
            Some(r) => NodeCommand::DestroyNode(*r),
            None => return,
        },
    };
    cmds.node_commands.push(cmd);
}

pub fn apply_class_name(cmds: &mut UserCommands, command: (Entity, ClassName), play_state: &XHashMap<Entity, Entity>) {
    let (n, class_name) = command;
    let node = match play_state.get(&n) {
        Some(r) => *r,
        None => return,
    };
    cmds.class_commands.push((node, class_name.clone()));
}
pub fn apply_fragment_command(cmds: &mut UserCommands, fragment_commands: &FragmentCommand, play_state: &XHashMap<Entity, Entity>) {
    let r: Vec<Entity> = fragment_commands
        .entitys
        .iter()
        .map(|r| {
            if let None = play_state.get(r) {
                log::warn!("xxxxxxx=========={:?}", r);
            }
            *play_state.get(r).unwrap()
        })
        .collect();

    cmds.fragment_commands.push(FragmentCommand {
        key: fragment_commands.key,
        entitys: r,
    });
}

pub fn apply_init_command(cmds: &mut UserCommands, node_init_command: (Entity, NodeTag), play_state: &XHashMap<Entity, Entity>) {
    let (entity, tag) = node_init_command;
    if let Some(entity) = play_state.get(&entity) {
        cmds.init_node(*entity, tag);
    }
}

pub fn apply_style_command(cmds: &mut UserCommands, s: &StyleCmd, play_state: &XHashMap<Entity, Entity>) {
        let entity = if let Some(entity) = play_state.get(&s.entity) { entity } else { return; };
        let class_mate = style_attr_list_to_buffer(play_state, &mut cmds.style_commands.style_buffer, &mut s.values.clone(), s.values.len());
        // log::warn!("style_commands:{:?}", s);
        cmds.style_commands
            .commands
            .push((*entity, class_mate.start, class_mate.end, None));
}

pub fn apply_nother_cmd(cmds: &mut UserCommands, other_command: &CmdType, play_state: &XHashMap<Entity, Entity>) {
    match other_command {
        CmdType::RuntimeAnimationBindCmd(r) => {
            let mut r = r.clone();
            let node = match play_state.get(&r.2) {
                Some(r) => *r,
                None => return,
            };
            r.2 = node;
            cmds.push_cmd(r);
        }
        CmdType::ComponentCmd(r) => {
            let mut r = r.clone();
            let node = match play_state.get(&r.1) {
                Some(r) => *r,
                None => return,
            };
            r.1 = node;
            cmds.push_cmd(r);
        }
        CmdType::NodeCmdViewport(r) => {
            let mut r = r.clone();
            let node = match play_state.get(&r.1) {
                Some(r) => *r,
                None => return,
            };
            r.1 = node;

            // if let Some(view_port) = play_state.view_port() {
            //     r.0 = Viewport(view_port);
            // }
            cmds.push_cmd(r);
        }
        CmdType::NodeCmdRenderTargetType(r) => {
            let mut r = r.clone();
            let node = match play_state.get(&r.1) {
                Some(r) => *r,
                None => return,
            };
            r.1 = node;
            cmds.push_cmd(r);
        }
        CmdType::NodeCmdRenderClearColor(r) => {
            // let mut r = r.clone();
            // let node = match play_state.get(&r.1) {
            //     Some(r) => r,
            //     None => continue,
            // };
            // r.1 = node;
            cmds.push_cmd(r.clone());
        }
        CmdType::NodeCmdRenderRenderDirty(r) => {
            cmds.push_cmd(r.clone());
        }
        CmdType::NodeCmdRenderNodeBundle(r) => {
            let mut r = r.clone();
            let node = match play_state.get(&r.1) {
                Some(r) => *r,
                None => return,
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
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::StrokeDasharrayCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgShapeCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgStrokeWidthCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgShapeWidthCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgShapeHeightCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgShapeXCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgShapeYCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
        // CmdType::SvgColorCmd(r) => {
        //     let mut r = r.clone();
        //     let node = match play_state.get(&r.0) {
        //         Some(r) => r,
        //         None => continue,
        //     };
        //     r.0 = node;
        //     cmds.push_cmd(r);
        // },
    };
}
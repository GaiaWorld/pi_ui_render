//! zindex系统
//! zindex的[min max), 采用Range, 左闭右开区间。
//! 预计每个节点自身占据Z_SELF个数，Z_SELF一般等于3.
//! 每节点考虑会有两边空间隔及自身， Z_SPLIT=3， Count个节点最多产生Count*Z_SPLIT个段。
//! 判断节点的zrange是否足够，在于全部的子节点数量(Count+1)*Z_SELF 不小于zrange。
//! 分配的间隔： S = (zrange-(Count+1)*Z_SELF)/(Count*Z_SPLIT), 为整数。 每个空间隔和节点都会加上这个S.
//! 分配节点的range为: 自身空间(S+Z_SELF) + 子节点及间隔空间(Count*(S*Z_SPLIT+Z_SELF))
//! 设计分配如下： 如果父容器为 0 6.
//! 子节点为1个的话，间隔为0： Empty(3,3), Node(3,6), Empty(6,6).
//! 设计分配如下： 如果父容器为 0 9.
//! 子节点为2个的话，间隔为0： Empty(3,3), Node(3,6), Empty(6,6), Node(6,9), Empty(9,9).
//! 设计分配如下： 如果父容器为 0 9.
//! 递归子节点为2个的话，间隔为0： Empty(3,3), Node(3,9), Empty(9,9).
//!                               Empty(6,6), Node(6,9), Empty(9,9).
//! 设计分配如下： 如果父容器为 0 9.
//! 子节点为1个的话，间隔为1： Empty(3,4), Node(4,8), Empty(8,9).
//! 设计分配如下： 如果父容器为 0 15.
//! 子节点为2个的话，间隔为1： Empty(3,4), Node(4,8), Empty(8,9), Node(9,13), Empty(13,15).
//! 递归子节点为2个的话，间隔为1： Empty(3,4), Node(4,14), Empty(14,15).
//!                               Empty(7,8), Node(8,12), Empty(12,13).
//!
//! 判断节点脏时，首先收集当前节点排序环境下的子节点，排序，然后：
//! 一类是节点重算zrange，重置全部的子节点的zrange。
//! 另一类是父节点下的子节点局部比较：顺序找到没有脏标志的节点，将其前面的节点重算zrange，继续选择没有脏标志的节点。 需要保证，前后节点区间的zrange能装的下所在的递归子节点，如果装不下，则扩大区间。
//!
//! # 注意
//! 本系统能够计算ZRange的前提是，ZRange组件必须存在于节点上，本系统不会新增ZRange组件

use std::ops::Range;

use pi_style::style::StyleType;
use pi_world::event::{ComponentAdded, ComponentChanged, Event};
use pi_world::fetch::Ticker;
use pi_world::prelude::{With, Query, Entity};
use pi_bevy_ecs_extend::prelude::{DirtyMark, EntityTree, Layer, LayerDirty, OrInitSingleRes, OrInitSingleResMut};

use pi_null::Null;
use pi_world::single_res::SingleRes;

use crate::components::calc::{style_bit, EntityKey, StyleBit, StyleMarkType, ZRange};
use crate::components::user::ZIndex;
use crate::resource::{GlobalDirtyMark, IsRun, OtherDirtyType};

use super::user_setting::StyleChange;
use super::world_matrix::Empty;

/// 如果节点设置zindex为auto，则自身zindex为-1
const Z_AUTO: isize = -1;
/// 节点zindex的最大区间
// const Z_MAX: usize = 16;//usize::MAX;
const Z_MAX: usize = std::u32::MAX as usize;
/// 每个节点自身占用的zindex区间大小
const Z_SELF: usize = 1;
/// 子节点将区间劈分成3段，自身在中间段
const Z_SPLIT: usize = 3;

pub struct CalcZindex;

lazy_static! {
	pub static ref ZINDEX_DIRTY: StyleMarkType = style_bit()
		.set_bit(StyleType::ZIndex as usize)
		.set_bit(OtherDirtyType::NodeTreeAdd as usize);
}

pub fn zindex_change(mark: SingleRes<GlobalDirtyMark>) -> bool {
	mark.mark.has_any(&*ZINDEX_DIRTY)
}

/// 根据层脏，从上到下，计算并设置节点的ZRange
pub fn calc_zindex(
    dirty_list: Event<StyleChange>,
    mut layer_dirty: LayerDirty<With<Empty>>,
    query_dirty: Query<(Ticker<&Layer>, Option<Ticker<&ZIndex>>)>,
    query_dirty1: Query<&crate::components::user::Opacity>,
    query: Query<&ZIndex>,
    
    mut global_mark: OrInitSingleResMut<GlobalDirtyMark>,
    tree: EntityTree,
    // mut dirtys: LayerDirty<((Changed<Layer>, Changed<ZIndex>), With<Size>)>,
    mut ranges: Query<&mut ZRange>,
    chaned: ComponentChanged<ZIndex>,
    added: ComponentAdded<ZIndex>,
	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}

    if global_mark.get(OtherDirtyType::NodeTreeAdd as usize).as_deref() == Some(&false) && chaned.len() == 0 && added.len() == 0{
        return;
    }
    if global_mark.get(OtherDirtyType::NodeTreeAdd as usize).as_deref() == Some(&true) {
        for i in dirty_list.iter() {
        
            if let Ok((layer, zindex)) = query_dirty.get(i.0) {
                // println!("calc_zindex1============{:?}, {:?}", i.0, (!layer.layer().is_null(), layer.is_changed(), zindex.is_changed()));
                if !layer.layer().is_null() && (layer.is_changed() || zindex.map_or(false, |zindex| {zindex.is_changed()}) ) {
                    
                    layer_dirty.mark(i.0);
                }
            }
        }
        chaned.mark_read();
        added.mark_read();
    } else {
        for i in chaned.iter().chain(added.iter()) {
        
            if let Ok((layer, _zindex)) = query_dirty.get(*i) {
                // println!("calc_zindex1============{:?}, {:?}", i.0, (!layer.layer().is_null(), layer.is_changed(), zindex.is_changed()));
                if !layer.layer().is_null() {
                    
                    layer_dirty.mark(*i);
                }
            }
        }
    }
  
    

    // zindex发生变化， 实例需要重新排序
	if layer_dirty.count() > 0 {
        global_mark.mark.set(OtherDirtyType::InstanceCount as usize, true);
        log::debug!("node_changed5============");
	} else {
        return;
    }

    // println!();

    let mut vec: Vec<ZSort> = vec![];
    for (id, mark, _) in layer_dirty.iter_manual() {
        match tree.get_up(id) {
            Some(up) if !up.parent().is_null() => {
                // log::debug!("calc_zindex======node: {:?}, parent: {:?}, layer: {:?} ", id, up.parent(), tree.get_layer(id));
                let parent = up.parent();
                // 找到能容纳所有子节点的父节点
                // parent节点zindex为AUTO，需要递归向上找到一个不是AUTO的节点, 以该节点作为布局环境，进行z布局
                // 如果parent节点无法容纳三倍子节点， 也需要向上递归，找到能容纳三倍子节点的节点作为布局环境进行z布局
                let (parent1, children_count, zrange, local) = get_parent(&query, &tree, &ranges, parent);
                // 收集父节点排序环境下的子节点
                collect(&query, &tree, &mut vec, parent1, 0);
                // 排序
                vec.sort();
                
                // println!("---------local:{}, {:?}", local, vec);
                if local {
                    // 如果是可以进行局部比较
                    local_reset(&query, &tree, mark, &mut ranges, &mut vec, children_count, zrange, &query_dirty1);
                } else {
                    // 否则父节点重新设置zrange
                    reset(&query, &tree, mark, &mut ranges, &mut vec, 0, children_count, zrange, &query_dirty1);
                }
            }
            _ => {
                // 根节点设置为最大值
                let _ = ranges.get_mut(id).map(|mut r| {
                    *r = ZRange(Range { start: 0, end: Z_MAX });
                });
            }
        }
    }
}


/// 获得能装下全部子节点的父节点
fn get_parent(query: &Query<&ZIndex>, tree: &EntityTree, ranges: &Query<&mut ZRange>, mut node: Entity) -> (Entity, usize, ZRange, bool) {
    let mut local = true;
    // println!("node:{:?}, ", &node);
    loop {
        if let Ok(z) = query.get(node) {
            // println!("z===={:?}", (node, z));
            if z.0 == Z_AUTO {
                // 如果该节点设置为Z_AUTO，则没有自己的排序环境，继续向父节点寻找
                node = tree.up(node).parent();
                // 有可能父不存在， 则直接将该节点当做非auto的节点处理
                if !EntityKey(node).is_null() {
                    continue;
                }
            }
        }

        let children_count = tree.down(node).count();
        let range = match ranges.get(node) {
            Ok(r) => r.clone(),
            _ => ZRange::default(),
        };

        // log::error!("get_parent======node: {:?}, parent: {:?}, down: {:?}, layer: {:?}, z_index: {:?}, z_range: {:?} ", node, tree.up(node).parent(), tree.down(node), tree.get_layer(node), query.get(node), range);
        if range.end - range.start >= children_count + 1 {
            return (node, children_count, range, local);
        }
        // println!("node range:{:?}, children_count:{}", (node, range, ranges.get(node)), children_count);
        // 节点的范围应该包含自身和递归子节点的z范围

        node = tree.up(node).parent();
        local = false // 因为父节点上没有脏标记，所以无法使用局部脏算法，只能全部排序
    }
}

/// 节点排序对象， 依次比较zindex, 自身所在位置
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct ZSort {
    z_index: isize,
    index: usize,
    node: EntityKey,
    children_count: usize,
}
impl ZSort {
    fn new(z_index: isize, index: usize, node: Entity, children_count: usize) -> Self {
        ZSort {
            z_index,
            index,
            node: EntityKey(node),
            children_count,
        }
    }
}
/// 收集父节点排序环境下的子节点
fn collect(query: &Query<&ZIndex>, tree: &EntityTree, vec: &mut Vec<ZSort>, parent: Entity, mut index: usize) -> usize {
    if let Some(down) = tree.get_down(parent) {
        for child in tree.iter(down.head()) {
            // 获得该节点的zindex
            let z = if let Ok(z) = query.get(child) {
                if z.0 == Z_AUTO {
                    // 如果该节点设置为Z_AUTO，则没有自己的排序环境，继续遍历其子节点
                    vec.push(ZSort::new(z.0, index, child, 0));
                    index = collect(query, tree, vec, child, index + 1);
                    continue;
                }
                z.0
            } else {
                0
            };
            // 获得该节点的递归子节点数
            let c = if let Some(down) = tree.get_down(child) { down.count() } else { 0 };
            vec.push(ZSort::new(z, index, child, c));
            index += 1;
        }
    }
    index
}

#[inline]
fn get_or_default<T: Clone + Default>(query: &Query<&mut T>, id: Entity) -> T {
    match query.get(id) {
        Ok(r) => r.clone(),
        _ => T::default(),
    }
}

/// 脏状态
#[derive(Debug)]
struct Dirty {
    children_count: usize, // 子节点总数量
    // begin、count、start分别描述了同一个节点的三个不同属性，
    begin: usize, // 在子元素数组中的索引，
    count: usize, // 顺序扫描， 统计了上次扫描到的不脏的节点到当前节点范围内的所有子节点
    start: usize, // z值起始范围
}
/// 父节点下的子节点局部比较
fn local_reset(
    query: &Query<&ZIndex>,
    tree: &EntityTree,
    mark: &mut DirtyMark,
    ranges: &mut Query<&mut ZRange>,
    vec: &mut Vec<ZSort>,
    children_count: usize,
    mut zrange: ZRange,
    query_dirty1: &Query<&crate::components::user::Opacity>,
) {
    fn empty(_mark: &mut DirtyMark, _node: &Entity) {}
    zrange.start += Z_SELF;
    // 脏状态
    let mut dirty = Dirty {
        children_count,
        count: 0,
        begin: 0,
        start: zrange.start,
    };
    let len = vec.len();
    for i in 0..len {
        let id = vec[i].node;
        // 获得当前节点及子节点的数量
        let cur_count = vec[i].children_count + 1;
        // 寻找修改的节点
        // 清理脏标志，这样层脏迭代器就不会弹出这个节点
        // println!("mark clear11, {}", vec[i].node.local().offset());
        if mark.remove(&id).is_some() {
            // println!("mark clear, {}", vec[i].node.local().offset());
            // 如果节点脏了， 统计到dirty的数量中
            dirty.count += cur_count;
            continue;
        }
        // 直到找到了一个未被修改的节点，下面成这个未被修改的节点为当前节点
        // 获得当前节点的zrange
        let cur_range = get_or_default(ranges, *id);

        log::debug!("local_reset====id: {:?}, zrange: {:?}, dirty: {:?}, cur_range: {:?}, cur_count: {:?}, down: {:?}, i: {:?}, len: {:?}", id, zrange, dirty, cur_range, cur_count, tree.get_down(*id), i, len);
        log::debug!("local_reset====branch1: {:?}, 2: {:?}, 3: {:?}", 
            zrange.end - cur_range.end < (dirty.children_count - dirty.count - cur_count) * Z_SELF, 
            dirty.count == 0, 
            cur_range.end < dirty.start || cur_range.end - dirty.start < (dirty.count + cur_count) * Z_SELF
        );
        // 判断当前节点右边（不包含当前节点）能否放下，如果不行，则继续（右边z范围不能容纳右边的所有子节点， 将当前节点卷入脏统计，并继续）
        if zrange.end - cur_range.end < (dirty.children_count - dirty.count - cur_count) * Z_SELF {
            dirty.count += cur_count;
            continue;
        }
        // 右边已经能容纳右边的节点，而当前统计的脏节点数量为0， 由于当前节点不脏，如果cur_range.start >= dirty.start， 则不需要处理当前节点
        if dirty.count == 0 && cur_range.start >= dirty.start {
            dirty.begin = i + 1;
            dirty.start = cur_range.end;
            continue;
        }
        // 右边已经能容纳右边的节点，如果左边（包含当前节点）不能容纳左边（包含当前节点）的节点 则将当前节点卷入脏统计，并继续
        if cur_range.end < dirty.start || cur_range.end - dirty.start < (dirty.count + cur_count) * Z_SELF {
            dirty.count += cur_count;
            continue;
        }

        //
        let (r, start, end) = if cur_range.start - dirty.start < dirty.count * Z_SELF {
            // 不含当前节点的情况下， 左侧需要调整容量无法容纳左侧需要调整节点，则将当前节点自身纳入脏统计
            dirty.count += cur_count;
            (ZRange(dirty.start..cur_range.end), dirty.begin, i + 1) // 含自身
        } else {
            (ZRange(dirty.start..cur_range.start), dirty.begin, i)
        };
        // // 前面有被修改节点，则获取脏段
        // let r = dirty_range(ranges, vec, zrange.start, range.start, &mut dirty);
        // dirty.start = range.end;
        log::debug!("local_reset====start: {:?}, end: {:?}, zrange: {:?}, dirty: {:?}", &vec[start].node, &vec[end - 1].node, zrange, dirty);
        // 重置脏段
        range_set(query, tree, mark, ranges, vec, start, end, dirty.count, r, empty, query_dirty1);
        // 将总子节点数量减去已经处理的数量
        dirty.children_count -= dirty.count;
        dirty.count = 0;
        dirty.begin = i + 1;
        dirty.start = cur_range.end;
    }
    // println!("dirty.count, {}", dirty.count);
    if dirty.count > 0 {
        log::debug!("local_reset1====start: {:?}, end: {:?}, zrange: {:?}, dirty: {:?}", &vec[dirty.begin as usize].node, &vec[len - 1].node, zrange, dirty);
        // 前面有被修改节点，则获取脏段
        // let r = dirty_range(ranges, vec, zrange.start, zrange.end, &mut dirty);
        range_set(
            query,
            tree,
            mark,
            ranges,
            vec,
            dirty.begin as usize,
            len,
            dirty.count,
            ZRange(dirty.start..zrange.end),
            empty,
            query_dirty1,
        );
    }
    // 清空
    vec.clear();
}
// /// 获取脏段，如果左边都可以放下，则返回true，否则返回false
// fn dirty_range(ranges: &Query<&mut ZRange>, vec: &Vec<ZSort>, parent_start: usize, dirty_end: usize, dirty: &mut Dirty) -> ZRange {
//     // println!("dirty_range, parent_start:{}, dirty_end:{}, dirty:{:?}", parent_start, dirty_end, dirty);
//     // 然后判断左边能否放下， 放不下， 则尝试向左移动，再次尝试能否放下
//     loop {
// 		// log::debug!("dirty======{:?}, {:?}, {:?}", dirty, dirty_end, Z_SELF);
//         // 判断左节点端及其子节点，都能被装下
//         if dirty_end - dirty.start >= dirty.count * Z_SELF {
//             return ZRange(Range {
//                 start: dirty.start,
//                 end: dirty_end,
//             });
//         }
//         if dirty.begin < 0 {
//             dirty.start = parent_start;
//         } else {
//             dirty.start = get_or_default(ranges, *vec[dirty.begin as usize].node).end;
// 			dirty.count += vec[dirty.begin as usize].children_count + 1;
// 			dirty.begin -= 1;
// 		}
//     }
// }
/// 设置子节点数组中一段节点的ZRange，并递归设置子节点的ZRange
fn range_set(
    query: &Query<&ZIndex>,
    tree: &EntityTree,
    mark: &mut DirtyMark,
    ranges: &mut Query<&mut ZRange>,
    vec: &mut Vec<ZSort>,
    begin: usize,
    end: usize,
    children_count: usize,
    mut zrange: ZRange,
    func: fn(&mut DirtyMark, &Entity),
    query_dirty1: &Query<&crate::components::user::Opacity>,
) {
    // println!("range set: zrange:{:?}, begin: {}, end: {}, count: {}", zrange, begin, end, children_count);
    // 获得间隔s
    let s = (zrange.end - zrange.start - children_count * Z_SELF) / (children_count * Z_SPLIT);
    zrange.start += s;
    for i in begin..end {
        let count = vec[i].children_count;
        let node = vec[i].node;
        func(mark, &node);
        // 分配节点的range为: 自身空间(S+Z_SELF) + 子节点及间隔空间(Count*(S*Z_SPLIT+Z_SELF))
        let r = ZRange(Range {
            start: zrange.start,
            end: zrange.start + s + Z_SELF + count * (s * Z_SPLIT + Z_SELF),
        });
        // log::debug!("range_set========zrange: {:?}, children_count: {:?}, s: {:?}, r{:?}, count: {:?}, begin: {}, end: {}", zrange, children_count, s, r, count, begin, end);
        zrange.start = r.end + s;
        set(query, tree, mark, ranges, vec, *node, count, r, query_dirty1);
    }
}
/// 父节点下的子节点全部重置zrange
fn reset(
    query: &Query<&ZIndex>,
    tree: &EntityTree,
    mark: &mut DirtyMark,
    ranges: &mut Query<&mut ZRange>,
    vec: &mut Vec<ZSort>,
    index: usize,
    children_count: usize,
    mut zrange: ZRange,
    query_dirty1: &Query<&crate::components::user::Opacity>,
) {

    zrange.start += Z_SELF;
    let len = vec.len();
    fn mark_remove(mark: &mut DirtyMark, node: &Entity) {
        // 清理脏标志，这样层脏迭代器就不会弹出这个节点
        mark.remove(&node);
    }
    
    // log::debug!("reset========list: {:?}", &vec[index..len]);
    range_set(query, tree, mark, ranges, vec, index, len, children_count, zrange, mark_remove, query_dirty1);
    // 清空
    vec.truncate(index);
}

/// 设置指定节点的ZRange，并递归设置子节点的ZRange
fn set(
    query: &Query<&ZIndex>,
    tree: &EntityTree,
    mark: &mut DirtyMark,
    ranges: &mut Query<&mut ZRange>,
    vec: &mut Vec<ZSort>,
    node: Entity,
    children_count: usize,
    zrange: ZRange,
    query_dirty1: &Query<&crate::components::user::Opacity>,
) {
    if let Ok(mut r) = ranges.get_mut(node) {
        if *r == zrange {
            return;
        }
        *r = zrange.clone();
        // log::debug!("set=========node: {:?}, z: {:?}", node, zrange);
        if children_count > 0 {
            let len = vec.len();
            // 收集该节点的排序环境下的子节点
            collect(&query, &tree, vec, node, 0);
           

            // 对新增的子节点进行排序
            let new_len = vec.len();
            // log::debug!("set1========list: {:?}", &vec[len..new_len]);
            vec[len..new_len].sort();
            // 递归设置zrange
            reset(query, tree, mark, ranges, vec, len, children_count, zrange, query_dirty1);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use pi_world::app::{App, CoreStage};
//     use pi_world::{
//         prelude::{Entity, EventWriter, World},
//         query::{Changed, QueryState},
//         system::{Local, SingleRes, SingleResMut, Resource, SystemState},
//     };
//     use pi_bevy_ecs_extend::{
//         prelude::{Down, EntityTreeMut, Layer, Up},
//         system_param::layer_dirty::ComponentEvent,
//     };
//     use pi_null::Null;

//     use crate::{
//         components::{
//             calc::{EntityKey, ZRange},
//             user::ZIndex,
//         },
//         system::node::z_index::calc_zindex,
//     };

//     #[derive(Resource, Deref)]
//     pub struct RootNode(Entity);

//     fn add(v: &mut isize) -> isize {
//         *v = *v + 1;
//         *v
//     }

//     fn init_1(
//         world: &mut World,
//         entity_tree: &mut SystemState<EntityTreeMut>,
//         event_writer: &mut SystemState<EventWriter<ComponentEvent<Changed<ZIndex>>>>,
//         root: &mut SystemState<SingleResMut<RootNode>>,
//     ) {
//         let root = **root.get_mut(world);
//         entity_tree.get_mut(world).insert_child(root, *EntityKey::null(), 0);

//         let mut i = 0;
//         // 插入2个节点作为子节点,以根节点作为父节点
//         let id = world
//             .spawn((ZIndex(add(&mut i)), ZRange::default(), Up::default(), Down::default(), Layer::default()))
//             .id();
//         entity_tree.get_mut(world).insert_child(id, root, 0);
//         event_writer.get_mut(world).send(ComponentEvent::new(id));

//         let id = world
//             .spawn((ZIndex(add(&mut i)), ZRange::default(), Up::default(), Down::default(), Layer::default()))
//             .id();
//         entity_tree.get_mut(world).insert_child(id, root, 0);
//         event_writer.get_mut(world).send(ComponentEvent::new(id));
//     }

//     fn init_2(
//         world: &mut World,
//         entity_tree: &mut SystemState<EntityTreeMut>,
//         root: &mut SystemState<SingleRes<RootNode>>,
//         event_writer: &mut SystemState<EventWriter<ComponentEvent<Changed<ZIndex>>>>,
//         mut local: Local<usize>,
//     ) {
//         *local += 1;
//         if *local != 2 {
//             return;
//         }


//         let root = **root.get_mut(world);
//         let id = world
//             .spawn((ZIndex(3), ZRange::default(), Up::default(), Down::default(), Layer::default()))
//             .id();
//         // 插入1个节点作为子节点,以根节点作为父节点
//         entity_tree.get_mut(world).insert_child(id, root, 0);
//         event_writer.get_mut(world).send(ComponentEvent::new(id));
//     }

//     fn init_3(
//         world: &mut World,
//         entity_tree: &mut SystemState<EntityTreeMut>,
//         root: &mut SystemState<SingleRes<RootNode>>,
//         event_writer: &mut SystemState<EventWriter<ComponentEvent<Changed<ZIndex>>>>,
//         mut local: Local<usize>,
//     ) {
//         *local += 1;
//         if *local != 3 {
//             return;
//         }

//         let root = **root.get_mut(world);
//         let id = world
//             .spawn((ZIndex(4), ZRange::default(), Up::default(), Down::default(), Layer::default()))
//             .id();
//         // 插入1个节点作为子节点,以根节点作为父节点
//         entity_tree.get_mut(world).insert_child(id, root, 0);
//         event_writer.get_mut(world).send(ComponentEvent::new(id));
//     }


//     #[test]
//     fn test() {
//         env_logger::Builder::default().filter(None, log::LevelFilter::Warn).init();

//         let mut app = App::default();
//         app.add_event::<ComponentEvent<Changed<ZIndex>>>();

//         let mut query = app.world.query::<(Entity, Option<&ZIndex>, &ZRange)>();

//         let root = app.world.spawn((ZRange(0..16), Up::default(), Down::default(), Layer::default())).id();

//         app.insert_single_res(RootNode(root))
//             .add_startup_system(init_1) // 插入根节点；插入前两个实体，以根节点作为父节点
//             .add_system_to_stage(CoreStage::PreUpdate, init_2) // 插入第3个实体，以根节点作为父节点
//             .add_system_to_stage(CoreStage::PreUpdate, init_3) // 插入第4个实体，以根节点作为父节点
//             .add_system(UiStage, calc_zindex)
//             .update();
//         asset(&mut app.world, &mut query, vec![(0, (0, 16)), (1, (4, 8)), (2, (9, 13))]);
//         println!("------------------------");


//         app.update();
//         asset(&mut app.world, &mut query, vec![(0, (0, 16)), (1, (4, 8)), (2, (9, 13)), (3, (13, 16))]);
//         println!("------------------------");

//         app.update();
//         asset(
//             &mut app.world,
//             &mut query,
//             vec![(0, (0, 16)), (1, (3, 6)), (2, (6, 9)), (3, (9, 12)), (4, (12, 15))],
//         );
//     }

//     fn asset(world: &mut World, query: &mut QueryState<(Entity, Option<&ZIndex>, &ZRange)>, result: Vec<(usize, (usize, usize))>) {
//         for (e, z, r) in query.iter_mut(world) {
//             let i = &result[e.index() as usize];
//             println!("=========, id:{:?}, z_index:{:?}, result: {:?}, expect: {:?}", e.index(), z, r, i.1);
//             assert_eq!(i.1 .0, r.0.start);
//             assert_eq!(i.1 .1, r.0.end);
//         }
//     }
// }

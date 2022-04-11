//! zindex系统
//! zindex的[min max), 采用Range, 开闭区间。
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

use std::ops::Range;

use pi_ecs::entity::Entity;
use pi_ecs::prelude::{Query, Write};
use pi_ecs::query::filter_change::Changed;
use pi_ecs::storage::{LocalVersion, SecondaryMap};
use pi_ecs_utils::prelude::{EntityTree, LayerDirty};
use pi_slotmap_tree::Storage;

use crate::components::calc::ZRange;
use crate::components::user::{Node, ZIndex};

/// 如果节点设置zindex为auto，则自身zindex为-1
const Z_AUTO: isize = -1;
/// 节点zindex的最大区间
const Z_MAX: usize = 12;//usize::MAX;
/// 每个节点自身占用的zindex区间大小
const Z_SELF: usize = 3;
/// 子节点将区间劈分成3段，自身在中间段
const Z_SPLIT: usize = 3;

/// 根据层脏，从上到下，计算并设置节点的ZRange
pub fn calc_zindex(
    query: Query<Node, Option<&ZIndex>>,
    tree: EntityTree<Node>,
    dirtys: LayerDirty<Node, Changed<ZIndex>>,
    mut ranges: Query<Node, Write<ZRange>>,
) {
    let mut vec: Vec<ZSort> = vec![];
    for (id, mark) in dirtys.iter_manual() {
        println!("dirty:{:?}", id);
        match tree.get_up(id) {
            Some(up) => {
                let parent = up.parent();
                // 找到能容纳所有子节点的父节点
                let (parent1, children_count, zrange, local) =
                    get_parent(&query, &tree, &ranges, parent);
                // 收集父节点排序环境下的子节点
                collect(&query, &tree, &mut vec, parent1, 0);
                // 排序
                vec.sort();
                if local {
                    // 如果是可以进行局部比较
                    local_reset(
                        &query,
                        &tree,
                        mark,
                        &mut ranges,
                        &mut vec,
                        children_count,
                        zrange,
                    );
                } else {
                    // 否则父节点重新设置zrange
                    reset(
                        &query,
                        &tree,
                        mark,
                        &mut ranges,
                        &mut vec,
                        0,
                        children_count,
                        zrange,
                    );
                }
            }
            _ => {
                // 根节点设置为最大值
                ranges.get_mut(id).unwrap().write(ZRange(Range {
                    start: 0,
                    end: Z_MAX,
                }));
            }
        }
    }
}

/// 获得能装下全部子节点的父节点
fn get_parent(
    query: &Query<Node, Option<&ZIndex>>,
    tree: &EntityTree<Node>,
    ranges: &Query<Node, Write<ZRange>>,
    mut node: Entity,
) -> (Entity, usize, ZRange, bool) {
    let mut local = true;
    println!("node:{:?}, ", &node);
    loop {
        if let Some(z) = query.get_unchecked(node) {
            println!("node z:{:?}, ", z);
            if z.0 == Z_AUTO {
                // 如果该节点设置为Z_AUTO，则没有自己的排序环境，继续向父节点寻找
                node = tree.up(node).parent();
                continue;
            }
        }
        let children_count = tree.down(node).count();
        let range = ranges.get(node).unwrap();
        let range = range.get_or_default();
        println!("node range:{:?}, children_count:{}", range, children_count);
        // 节点的范围应该包含自身和递归子节点的z范围
        if range.end - range.start > (children_count + 1) * Z_SELF {
            return (node, children_count, range.clone(), local);
        }
        node = tree.up(node).parent();
        local = false
    }
}

/// 节点排序对象， 依次比较zindex, 自身所在位置
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct ZSort {
    z_index: isize,
    index: usize,
    node: Entity,
    children_count: usize,
}
impl ZSort {
    fn new(z_index: isize, index: usize, node: Entity, children_count: usize) -> Self {
        ZSort {
            z_index,
            index,
            node,
            children_count,
        }
    }
}
/// 收集父节点排序环境下的子节点
fn collect(
    query: &Query<Node, Option<&ZIndex>>,
    tree: &EntityTree<Node>,
    vec: &mut Vec<ZSort>,
    parent: Entity,
    mut index: usize,
) -> usize {
    if let Some(down) = tree.get_down(parent) {
        for child in tree.iter(down.head()) {
            // 获得该节点的zindex
            let z = if let Some(z) = query.get_unchecked(child) {
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
            let c = if let Some(down) = tree.get_down(child) {
                down.count()
            } else {
                0
            };
            vec.push(ZSort::new(z, index, child, c));
            index += 1;
        }
    }
    index
}

/// 脏状态
struct Dirty {
    children_count: usize,
    begin: usize,
    count: usize,
    zrange: ZRange,
}
/// 父节点下的子节点局部比较
fn local_reset(
    query: &Query<Node, Option<&ZIndex>>,
    tree: &EntityTree<Node>,
    mark: &mut SecondaryMap<LocalVersion, usize>,
    ranges: &mut Query<Node, Write<ZRange>>,
    vec: &mut Vec<ZSort>,
    children_count: usize,
    zrange: ZRange,
) {
    fn empty(_mark: &mut SecondaryMap<LocalVersion, usize>, _node: &Entity) {}
    let z_end = zrange.end;
    // 脏状态
    let mut dirty = Dirty {
        children_count,
        count: 0,
        begin: 0,
        zrange,
    };
    dirty.zrange.start += Z_SELF;
    let len = vec.len();
    for i in 0..len {
        // 寻找修改的节点
        // 清理脏标志，这样层脏迭代器就不会弹出这个节点
        if mark.remove(vec[i].node.local()).is_some() {
            dirty.count += vec[i].children_count + 1;
            continue;
        }
        // 找到了没有被修改的节点，获得其zrange
        let range = ranges.get(vec[i].node).unwrap();
        // 如果不克隆，可能有问题？
        let range = range.get_or_default();
        // 如果前面没有修改的节点，则跳过当前没有被修改的节点
        if dirty.count == 0 {
            dirty.begin = i + 1;
            dirty.zrange.start = range.end;
            continue;
        }
        // 前面有被修改节点，则获取脏段
        let cur_count = dirty_range(ranges, vec, i, &range, &mut dirty);
        if cur_count == 0 {
            let zrange = dirty.zrange.clone();
            dirty.zrange.start = range.end;
            dirty.zrange.end = z_end;
            // 重置脏段
            range_set(
                query,
                tree,
                mark,
                ranges,
                vec,
                dirty.begin,
                i,
                dirty.count,
                zrange,
                empty,
            );
            dirty.count = 0;
            dirty.begin = i + 1;
        } else {
            dirty.count += cur_count;
        }
    }
    if dirty.count > 0 {
        range_set(
            query,
            tree,
            mark,
            ranges,
            vec,
            dirty.begin,
            len,
            dirty.count,
            dirty.zrange,
            empty,
        );
    }
    // 清空
    vec.clear();
}
/// 获取脏段，如果左右都可以放下，则返回0，否则返回当前节点及子节点的数量
fn dirty_range(
    ranges: &Query<Node, Write<ZRange>>,
    vec: &Vec<ZSort>,
    index: usize,
    range: &ZRange,
    dirty: &mut Dirty,
) -> usize {
    // 获得当前节点及子节点的数量
    let cur_count = vec[index].children_count + 1;
    // 先判断当前节点能否放下其递归子节点，如果不行，则返回
    if range.end - range.start < cur_count * Z_SELF {
        return cur_count;
    }
    // 判断右边能否放下，如果不行，则返回
    if dirty.zrange.end - range.end < (dirty.children_count - dirty.count - cur_count) * Z_SELF {
        return cur_count;
    }
    // 然后判断左边能否放下， 放不下， 则尝试向左移动，再次尝试能否放下
    loop {
        // 判断左节点端及其子节点，都能被装下
        if range.start - dirty.zrange.start >= dirty.count * Z_SELF {
            dirty.children_count -= dirty.count;
            return 0;
        }
        if dirty.begin == 0 {
            return cur_count;
        }
        dirty.begin -= 1;
        dirty.count += vec[dirty.begin].children_count + 1;
        let r = ranges.get(vec[dirty.begin].node).unwrap();
        //let r = r.get_or_default();
        dirty.zrange.start = r.get_or_default().end;
    }
}
/// 设置子节点数组中一段节点的ZRange，并递归设置子节点的ZRange
fn range_set(
    query: &Query<Node, Option<&ZIndex>>,
    tree: &EntityTree<Node>,
    mark: &mut SecondaryMap<LocalVersion, usize>,
    ranges: &mut Query<Node, Write<ZRange>>,
    vec: &mut Vec<ZSort>,
    begin: usize,
    end: usize,
    children_count: usize,
    mut zrange: ZRange,
    func: fn(&mut SecondaryMap<LocalVersion, usize>, &Entity),
) {
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
            end: zrange.start + s + Z_SELF + children_count * (s * Z_SPLIT + Z_SELF),
        });
        zrange.start = r.end + s;
        set(query, tree, mark, ranges, vec, node, count, r);
    }
}
/// 父节点下的子节点全部重置zrange
fn reset(
    query: &Query<Node, Option<&ZIndex>>,
    tree: &EntityTree<Node>,
    mark: &mut SecondaryMap<LocalVersion, usize>,
    ranges: &mut Query<Node, Write<ZRange>>,
    vec: &mut Vec<ZSort>,
    index: usize,
    children_count: usize,
    mut zrange: ZRange,
) {
    zrange.start += Z_SELF;
    let len = vec.len();
    fn mark_remove(mark: &mut SecondaryMap<LocalVersion, usize>, node: &Entity) {
        // 清理脏标志，这样层脏迭代器就不会弹出这个节点
        mark.remove(node.local());
    }
    range_set(
        query,
        tree,
        mark,
        ranges,
        vec,
        index,
        len,
        children_count,
        zrange,
        mark_remove,
    );
    // 清空
    vec.truncate(index);
}

/// 设置指定节点的ZRange，并递归设置子节点的ZRange
fn set(
    query: &Query<Node, Option<&ZIndex>>,
    tree: &EntityTree<Node>,
    mark: &mut SecondaryMap<LocalVersion, usize>,
    ranges: &mut Query<Node, Write<ZRange>>,
    vec: &mut Vec<ZSort>,
    node: Entity,
    children_count: usize,
    zrange: ZRange,
) {
    let mut r = ranges.get_mut(node).unwrap();
    if *r.get_or_default() == zrange {
        return;
    }
    if children_count > 0 {
        r.write(zrange.clone());
        let len = vec.len();
        // 收集该节点的排序环境下的子节点
        collect(&query, &tree, vec, node, 0);
        // 对新增的子节点进行排序
        let new_len = vec.len();
        vec[len..new_len].sort();
        // 递归设置zrange
        reset(query, tree, mark, ranges, vec, len, children_count, zrange);
    } else {
        r.write(zrange);
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use pi_async::rt::{
        multi_thread::{MultiTaskRuntimeBuilder, StealableTaskPool},
        AsyncRuntime,
    };
    use pi_ecs::{
        entity::Entity,
        prelude::{
            Dispatcher, In, IntoSystem, Query, QueryState, SingleDispatcher, StageBuilder, System,
            World, Write,
        },
    };
    use pi_ecs_utils::prelude::{EntityTreeMut, Layer, NodeDown, NodeUp};
    use pi_null::Null;

    use crate::components::{
        calc::ZRange,
        user::{Node, ZIndex},
    };

    use super::calc_zindex;

    // 初始化，将所有节点以根节点作为父节点组织为树
    fn init_tree(root: In<Entity>, mut tree: EntityTreeMut<Node>, entitys: Query<Node, Entity>) {
        let r = root.0;
        for e in entitys.iter() {
            if e != r {
                tree.insert_child(e, r, std::usize::MAX);
            } else {
                tree.insert_child(e, Entity::null(), std::usize::MAX);
            }
        }
    }

    #[test]
    fn test() {
        // 创建world
        let mut world = World::new();

        // 创建原型
        world
            .new_archetype::<Node>()
            .register::<Layer>()
            .register::<NodeUp>()
            .register::<NodeDown>()
            .register::<ZIndex>()
            .register::<ZRange>()
            .create();

        // 派发器
        let dispatcher = get_dispatcher(&mut world);

        let mut entitys = Vec::new();
        let root = world.spawn::<Node>().id();

        //插入根节点
        entitys.push(root);
        let mut i = 1;
        // 插入三个节点作为子节点
        while i < 4 {
            let entity = world.spawn::<Node>().insert(ZIndex(i)).id();
            // 插入实体，以根节点作为父节点
            entitys.push(entity);

            i += 1;
        }

        // 组织为树结构
        let mut init_tree_sys = init_tree.system(&mut world);
        init_tree_sys.run(In(root));

        let mut query = world.query::<Node, (Entity, Option<&ZIndex>, &ZRange)>();

        // 测试计算
        dispatcher.run();
        asset(&mut world, &mut query, vec![(0, (0, 9)), ]);

        // 最后一个实体，添加一个缩放为0.5的Transform
        // dispatcher.run();
        // asset(&mut world, &mut query);
    }

    fn get_dispatcher(world: &mut World) -> SingleDispatcher<StealableTaskPool<()>> {
        let rt = AsyncRuntime::Multi(MultiTaskRuntimeBuilder::default().build());
        let system = calc_zindex.system(world);

        let mut stage = StageBuilder::new();
        stage.add_node(system);

        let mut stages = Vec::new();
        stages.push(Arc::new(stage.build()));
        let dispatcher = SingleDispatcher::new(stages, world, rt);

        dispatcher
    }

    fn asset(
        world: &mut World,
        query: &mut QueryState<Node, (Entity, Option<&ZIndex>, &ZRange)>,
        result: Vec<(isize, (usize, usize))>,
    ) {
        for (_e, z, r) in query.iter_mut(world) {
            log::debug!("=========, id:{:?}, r: {:?}", z, r);
            assert!(result.iter().any(|&i| {
                if i.0 == -1 && z.is_none() {
                    i.1.0 == r.0.start && i.1.1 == r.0.end
                }else if z.is_some() && i.0 == **z.unwrap() {
                    i.1.0 == r.0.start && i.1.1 == r.0.end
                }else{
                    false
                }                
             } ));
        }
    }
}

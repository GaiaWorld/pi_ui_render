//! 根据LayoutR, Tranform组件， 计算节点的世界矩阵
//!
//! ## 计算过程
//! 节点必须存在如下组件：
//! * LayoutResult
//! 节点可能存在如下组件：
//! * Transform
//!
//! Transform组件描述节点的变化，通常可能是以自身中心点（或其它点）为原点进行变换
//! 而LayoutR描述的节点矩形位置，是以父节点布局矩形的左上角为原点的坐标描述
//! 因此需要同一它们描述的原点坐标
//!
//! 本模块将Transform描述的变换转换到以父节点布局的矩形区域左上角为原点
//!
//! 设节点布局后的矩形区域：left_top_x, left_top_y, right_bottom_x, right_bottom_y
//! Transform的转化过程为：M(移动left_top_x、left_top_y) * M(Transform) * M(-left_top_x、-left_top_y)
//! 这样Transform的变幻矩阵就变为了以父节点布局的矩形左上角为原点的变化。
//!
//! 尽管Transform的转换过程稍显复杂，但大部分情况下，是不存在Transform组件的，因此不会计算这种转化，只需要计算自身布局属性包含的变化信息，世界矩阵计算的常数时间不会太长。
//!
//! 世界矩阵计算公式：
//! 	  ParentMatrix * TransformMatrix
//! 	或ParentMatrix
//!
//! ## 优化
//! ### 层次脏优化
//!
//! WorldMatrix的计算存在以下特性：
//! * Transform的变化和Layout的变化，会影响自身世界矩阵，但不会影响父节点、兄弟节点的世界矩阵（除非它们的对应属性发生改变）
//! * WorldMatrix除了受到自身Transform和Layout的影响，也收到父矩阵的影响
//!
//! 因此，在计算时，需要先计算父节点的世界矩阵（如果先计算子节点，在计算父节点，父节点的世界矩阵会再次作用到子节点，子节点不要重新算）
//!
//! 层次脏根据节点树，记录了修改节点的层，在遍历时优先遍历层较低的节点（父），同时递归遍历该节点下的所有子节点。
//!
//! ## 并行
//! 暂时无并行。
//!
//! 可以考虑： 当父矩阵计算完成后，父节点所有子节点所形成的子树，可以并行计算（他们依赖的父矩阵已经计算完毕）
use pi_world::event::{ComponentAdded, ComponentChanged, EventSender};
use pi_world::prelude::{ParamSet, Query, SingleResMut, Entity, With};
use pi_bevy_ecs_extend::prelude::{EntityTree, Layer, LayerDirty, OrInitSingleRes, OrInitSingleResMut, Up};

use pi_map::Map;
use pi_null::Null;
use pi_style::style::{Aabb2, StyleType};

use crate::components::calc::{EntityKey, IsRotate, LayoutResult, Quad, StyleMark, WorldMatrix};
use crate::components::user::{BoxShadow, Point2, TextShadow, Transform};
use crate::resource::{GlobalDirtyMark, IsRun, MatrixDirty, OtherDirtyType, QuadTree};
use crate::components::user::Vector4;


pub struct CalcMatrix;

#[derive(Debug)]
pub struct OldQuad {
    pub entity: Entity,
    pub root: Entity,
    pub quad: Quad,
}

// fn print_parent(idtree: &EntityTree<Node>, id: Id<Node>) {
//     let parent_id = idtree.get_up(id).map_or(Id::<Node>::null(), |up| up.parent());
//     if !parent_id.is_null() {
//         println!("parent======{:?}", parent_id);
//         print_parent(idtree, parent_id);
//     }
// }

pub const RENDER_TYPE: u32 = 
    ( 1 << StyleType::BackgroundColor as usize) |
    ( 1 << StyleType::BackgroundImage as usize) |
    ( 1 << StyleType::BorderColor as usize) |
    ( 1 << StyleType::BorderImage as usize) ; // |

    // ( 1 << StyleType::TextContent as usize); // TODO
pub struct Empty;
/// 计算世界矩阵
/// 世界矩阵以自身左上角为原点
pub fn cal_matrix(
    // dirtys1: Query<
    // (
    //     Ticker<&Layer>,
    //     Option<Ticker<&Transform>>,
    // )>,
    // dirty_list: Event<StyleChange>,
    mut layer_dirty: LayerDirty<With<Empty>>,
    // query_dirty: Query<(Ticker<&Layer>, Ticker<&LayoutResult>, Option<Ticker<&Transform>>, Option<Ticker<&TextShadow>>, Option<Ticker<&BoxShadow>>)>,
    layout_dirty: ComponentChanged<LayoutResult>,
    transform_dirty: ComponentChanged<Transform>,
    transform_add: ComponentAdded<Transform>,

    query: Query<(Option<&Transform>, &LayoutResult, &Up)>,

    // query11: Query<(Entity,  Option<Ticker<&Transform>>, Ticker<&Layer>, Ticker<&LayoutResult>), ((Changed<LayoutResult>, Changed<Layer>, Changed<Transform>), With<Size>)>,
    mut matrix_calc: ParamSet<(
        Query<(&LayoutResult, &WorldMatrix)>, 
        Query<(&mut WorldMatrix, &mut Quad, &mut IsRotate, &StyleMark, &Layer)>,
        Query<(&Quad, &LayoutResult, Option<&TextShadow>, Option<&BoxShadow>, &WorldMatrix)>,
    )>,
    // mut dirtys: LayerDirty<((Changed<LayoutResult>, Changed<Transform>), With<Size>)>,
    mut quad_tree: SingleResMut<QuadTree>,
	r: OrInitSingleRes<IsRun>,
	#[cfg(debug_assertions)]
	debug_entity: OrInitSingleRes<crate::resource::DebugEntity>,


    // node_box: Query<(&Quad, &LayoutResult, Option<&TextShadow>, Option<&BoxShadow>, &WorldMatrix)>,
    // down: Query<&Down>,
    // up: Query<&Up>,
    // layer: Query<&Layer>,
    // content_box: Query<&mut ContentBox>,
    mut layer_dirty1: OrInitSingleResMut<MatrixDirty>,
    entity_tree: EntityTree,
    mut global_dirty: SingleResMut<GlobalDirtyMark>,
    mut event_writer1: EventSender<OldQuad>,
) {
	if r.0 {
		return;
	}
    
    for i in layout_dirty.iter().chain(transform_dirty.iter()).chain(transform_add.iter()) {
        layer_dirty.mark(*i);
    }

    let layer_dirty1 = &mut layer_dirty1.0;


    if layer_dirty.count() > 0 {
        global_dirty.mark.set(OtherDirtyType::WorldMatrix as usize, true);
    } else {
        return;
    }

    // let time2 = pi_time::Instant::now();
    // println!("matrix time1========{:?}", ( time2 - time1));
    for id in layer_dirty.iter() { 
        layer_dirty1.marked_dirty(id, id, &entity_tree);
        // ii1.push(id);
        // if count == 1 {
		// 	log::warn!("matrix time0========{:?}", pi_time::Instant::now() - time1);
		// }
		// let time1 = pi_time::Instant::now();
        if let Ok((transform, layout, up)) = query.get(id) {

            let parent_id = up.parent();

            let width = layout.rect.right - layout.rect.left;
            let height = layout.rect.bottom - layout.rect.top;

            let matrix = if EntityKey(parent_id).is_null() {
                // 父为空，则其为根节点，其世界矩阵为单位阵
                let mut r = WorldMatrix::default();
                if let Some(transform) = transform {
                    // log::warn!("matrix time1========{:?}", transform);
                    r = r * WorldMatrix::form_transform_layout(
                        &transform.all_transform,
                        &transform.origin,
                        width,
                        height,
                        &Point2::new(layout.rect.left, layout.rect.top),
                    );
                }
                r
            } else {
                let p0 = &matrix_calc.p0();
                // 否则
                let (parent_layout, parent_world_matrix) = match p0.get(parent_id) {
                    Ok(r) => (&*r.0, &*r.1),
                    Err(_) => {
                        log::error!(
                            "calc matrix fail, parent matrix or layout is not exist!, id:{:?}, parent_id: {:?}",
                            id,
                            parent_id
                        );
                        return;
                    }
                };

                let offset = (layout.rect.left + parent_layout.border.left + parent_layout.padding.left, layout.rect.top + parent_layout.border.top + parent_layout.padding.top);
                match transform {
                    // transform存在时，根据transform和布局计算得到变换矩阵，再乘以父矩阵
                    Some(transform) => {
                        let r = parent_world_matrix
                            * WorldMatrix::form_transform_layout(
                                &transform.all_transform,
                                &transform.origin,
                                width,
                                height,
                                &Point2::new(offset.0, offset.1),
                            );
                        r
                    }
                    // transform不存在时，节点的变换矩阵可以直接由布局结果得出，世界矩阵计算更快，大部分情况也是走这条快速路径
                    None => {
                        let mut w = parent_world_matrix.clone();
                        w.translate(offset.0, offset.1, 0.0);
                        // log::warn!("matrix time2===={:?}, {:?}, {:?}", id, up.parent(), w);
                        w
                    }
                }
            };

            // use pi_key_alloter::Key;
            // if id.index() == 4 || id.index() == 3 {
                // log::warn!("matrx================id:{:?}, \nr:{:?}, \n:{:?}", 
                // id, &matrix, transform);
            // }
            // println!("matrix============={:?}, {:?} {:?}, {:?}, {:?}", id, down.head, layout, transform, matrix);
			// if count == 1 {
			// 	log::warn!("matrix time1========{:?}", pi_time::Instant::now() - time1);
			// }
            // 将计算结果写入组件
            match matrix_calc.p1().get_mut(id) {
                Ok((mut world_matrix, mut quad, mut is_rotate, style_mark, layer)) => {
					calc_quad(
						id,
						layout,
						&matrix,
						&mut quad,
						&mut quad_tree,
                        &mut event_writer1,
                        layer
					);
                   
					#[cfg(debug_assertions)]
					if id == debug_entity.0.0 {
						log::debug!("matrix=============id={:?}, \nlayout={:?}, \nmatrix={:?}, \nquad={:?}", id, layout, &matrix, &quad);
					}
                    // log::warn!("matrix=============id={:?}, parent: {:?}, layout={:?}, \nmatrix={:?}, \nquad={:?}, \ntransform={:?}", id, parent_id, layout, &*matrix, &*quad, transform);
                    if is_rotate.0 != matrix.1 {
                        is_rotate.0 = matrix.1;
                    }
                    *world_matrix = matrix;

                    // 标记渲染obj脏（优化性能， 不需要每个渲染obj的system在matrix变化的情况下总是迭代， 如果matrix修改， 但节点上没有任何渲染属性， 则其他system不需要迭代）
                    let mut m = style_mark.local_style.as_raw_slice()[0] & RENDER_TYPE;
                    if m > 0 {
                        global_dirty.as_raw_mut_slice()[0] |= m;
                    }
                    m = style_mark.class_style.as_raw_slice()[0] & RENDER_TYPE;
                    if m > 0 {
                        global_dirty.as_raw_mut_slice()[0] |= m;
                    }
                }
                Err(_) => {}
            };
        }
    }
    
    // let time3 = pi_time::Instant::now();
    

    // calc_content_box(&mut layer_dirty1, matrix_calc.p2(), down, up, layer, content_box);
    // let time4 = pi_time::Instant::now();
    // println!("matrix time========{:?}, calc_content_box: {:?}", (time3 - time1, time1 - time, i, j), time4 - time3);
    // if dirtys.count() > 0 {
	// 	log::warn!("start parent==========={:?}", (dirtys.count(), time3 - time2, time2 - time1));
	// }
	// if count == 1 {
	// 	log::warn!("matrix time========{:?}, {:?}", pi_time::Instant::now() - time1, time1 - time);
	// }
}

pub fn calc_quad(
    id: Entity,
    layout: &LayoutResult,
    world_matrix: &WorldMatrix,
    quad: &mut Quad,
    quad_tree: &mut QuadTree,
    event_writer1: &mut EventSender<OldQuad>,
    layer: &Layer,
) {
    let width = layout.rect.right - layout.rect.left;
    let height = layout.rect.bottom - layout.rect.top;

    let left_top = world_matrix * Vector4::new(0.0, 0.0, 0.0, 1.0);
    let right_bottom = world_matrix * Vector4::new(width, height, 0.0, 1.0);

    let item = if world_matrix.1 {
        let right_top = world_matrix * Vector4::new(width, 0.0, 0.0, 1.0);
        let left_bottom = world_matrix * Vector4::new(0.0, height, 0.0, 1.0);
        let min = Point2::new(
            left_top.x.min(right_top.x).min(left_bottom.x).min(right_bottom.x),
            left_top.y.min(right_top.y).min(left_bottom.y).min(right_bottom.y),
        );
    
        let max = Point2::new(
            left_top.x.max(right_top.x).max(left_bottom.x).max(right_bottom.x),
            left_top.y.max(right_top.y).max(left_bottom.y).max(right_bottom.y),
        );
    
        Quad::new(Aabb2::new(min, max))
    } else {
        Quad::new(Aabb2::new(Point2::new(
            left_top.x.min(right_bottom.x),
            left_top.y.min(right_bottom.y),
        ), Point2::new(
            left_top.x.max(right_bottom.x),
            left_top.y.max(right_bottom.y),
        )))
    };
    

    // let aabb = calc_bound_box(&Aabb2::new(Point2::new(0.0, 0.0), Point2::new(width, height)), world_matrix);

    // let item = Quad::new(aabb);
    // 在修改oct前，先发出一个旧的包围盒事件，一些sys能够通过监听该事件知道在修改前，quad的值（如脏区域系统，需要了解oct在修改之前的值，来更新脏区域）
    if let Some(r) = quad_tree.get(&EntityKey(id)) {
        event_writer1.send(OldQuad {
            entity: id,
            quad: r.clone(),
            root: layer.root(),
        });
    }
    // event_writer.send(ComponentEvent::new(id));

    quad_tree.insert(EntityKey(id), item.clone());
    log::trace!(target: format!("entity_{:?}", id).as_str(), "calc_quad={:?}", item);

    *quad = item;
}



// #[cfg(test)]
// pub mod test {

//     use pi_world::app::{App, CoreStage};
//     use pi_world::prelude::{Component, Entity, EventReader, EventWriter};
//     use pi_world::query::Changed;
//     use pi_world::system::{Commands, Query, SingleRes, SingleResMut, Resource};
//     use pi_bevy_ecs_extend::prelude::{Down, EntityTreeMut, Layer, Up};
//     use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
//     use pi_flex_layout::prelude::Rect;
//     use pi_map::Map;
//     use pi_null::Null;

//     use crate::components::calc::WorldMatrix;
//     use crate::components::calc::{EntityKey, LayoutResult, Quad};
//     use crate::components::user::{Transform, TransformFunc, Vector4};
//     use crate::resource::QuadTree;

//     use super::cal_matrix;

//     #[test]
//     fn test() {
//         // 创建world
//         env_logger::Builder::default().filter(None, log::LevelFilter::Warn).init();

//         let mut app = App::default();
//         let root = app.world.spawn(()).id();
//         app.add_event::<ComponentEvent<Changed<Transform>>>()
//             .add_event::<ComponentEvent<Changed<LayoutResult>>>()
//             .add_event::<ComponentEvent<Changed<Layer>>>()
//             .add_event::<ComponentEvent<Changed<Quad>>>()
//             .add_event::<Vec<Entity>>()
//             .insert_single_res(QuadTree::with_capacity(0))
//             .insert_single_res(AllEntitys(Vec::new()))
//             .insert_single_res(RootNode(root))
//             .add_startup_system(setup1)
//             .add_system(UiStage, init_tree)
//             .add_system_to_stage(CoreStage::PostUpdate, cal_matrix)
//             .add_system_to_stage(CoreStage::End, asset_matrix)
//             .add_system_to_stage(CoreStage::End, asset_quad)
//             .update();

//         app.add_system_to_stage(CoreStage::PreUpdate, setup2).update();
//     }

//     #[derive(Resource, Deref)]
//     pub struct RootNode(Entity);

//     #[derive(Resource, Deref)]
//     pub struct AllEntitys(Vec<Entity>);

//     fn setup1(mut command: Commands, mut events: EventWriter<Vec<Entity>>, root: SingleRes<RootNode>) {
//         let mut entitys = Vec::new();
//         let root = command
//             .entity(root.0)
//             .insert((
//                 Up::default(),
//                 Down::default(),
//                 Layer::default(),
//                 WorldMatrix::default(),
//                 Quad::default(),
//                 LayoutResult {
//                     rect: Rect {
//                         left: 0.0,
//                         right: 1000.0,
//                         top: 0.0,
//                         bottom: 1000.0,
//                     },
//                     border: Rect {
//                         left: 0.0,
//                         right: 0.0,
//                         top: 0.0,
//                         bottom: 0.0,
//                     },
//                     padding: Rect {
//                         left: 0.0,
//                         right: 0.0,
//                         top: 0.0,
//                         bottom: 0.0,
//                     },
//                 },
//                 AbsolutePosition(Rect {
//                     left: 0.0,
//                     right: 1000.0,
//                     top: 0.0,
//                     bottom: 1000.0,
//                 }),
//             ))
//             .id();

//         //插入根节点
//         entitys.push(root);

//         let size = 50.0;
//         let mut left_top = 0.0;
//         let mut right_bottom;
//         // 插入三个节点作为子节点
//         for _i in 0..3 {
//             right_bottom = left_top + size;

//             let entity = command
//                 .spawn((
//                     Up::default(),
//                     Down::default(),
//                     Layer::default(),
//                     WorldMatrix::default(),
//                     Quad::default(),
//                     LayoutResult {
//                         rect: Rect {
//                             left: left_top,
//                             right: right_bottom,
//                             top: left_top,
//                             bottom: right_bottom,
//                         },
//                         border: Rect {
//                             left: 0.0,
//                             right: 0.0,
//                             top: 0.0,
//                             bottom: 0.0,
//                         },
//                         padding: Rect {
//                             left: 0.0,
//                             right: 0.0,
//                             top: 0.0,
//                             bottom: 0.0,
//                         },
//                     },
//                     AbsolutePosition(Rect {
//                         left: left_top,
//                         right: right_bottom,
//                         top: left_top,
//                         bottom: right_bottom,
//                     }),
//                 ))
//                 .id();
//             // 插入实体，以根节点作为父节点
//             entitys.push(entity);

//             left_top += size;
//         }
//         events.send(entitys);
//     }

//     /// 最后一个实体，添加一个缩放为0.5的Transform
//     fn setup2(mut command: Commands, all_entitys: SingleRes<AllEntitys>, mut event_writer: EventWriter<ComponentEvent<Changed<Transform>>>) {
//         let last_entity = all_entitys.0[all_entitys.0.len() - 1];
//         let mut t = Transform::default();
//         t.funcs.push(TransformFunc::Scale(0.5, 0.5));
//         command.entity(last_entity).insert((
//             t,
//             AbsolutePosition(Rect {
//                 // 测试矩阵计算, 最后一个实体组件缩放为原来的0.5
//                 left: 112.5,
//                 right: 137.5,
//                 top: 112.5,
//                 bottom: 137.5,
//             }),
//         ));
//         event_writer.send(ComponentEvent::new(last_entity));
//     }

//     // 绝对位置,节点以左上为原点，经过布局、变化，得到的最终位置
//     #[derive(Deref, Debug, Component)]
//     pub struct AbsolutePosition(Rect<f32>);

//     // 初始化，将所有节点以根节点作为父节点组织为树
//     fn init_tree(root: SingleRes<RootNode>, mut tree: EntityTreeMut, mut entitys: EventReader<Vec<Entity>>, mut all_entitys: SingleResMut<AllEntitys>) {
//         let r = root.0;
//         for list in entitys.iter() {
//             all_entitys.0.extend_from_slice(list.as_slice());
//             for e in list.iter() {
//                 if *e != r {
//                     tree.insert_child(*e, r, std::usize::MAX);
//                 } else {
//                     tree.insert_child(*e, EntityKey::null().0, std::usize::MAX);
//                 }
//             }
//         }
//     }

//     fn asset_matrix(query: Query<(Entity, &WorldMatrix, &LayoutResult, &AbsolutePosition)>) {
//         // log::warn!("asset_matrix======");
//         for (_e, w, l, a_p) in query.iter() {
//             let left_top = w * Vector4::new(0.0, 0.0, 1.0, 1.0);
//             let right_bottom = w * Vector4::new(l.rect.right - l.rect.left, l.rect.bottom - l.rect.top, 1.0, 1.0);
//             // println!("e: {:?}, a_p: {:?}, left_top: {:?}, right_bottom: {:?}", _e, a_p, left_top, right_bottom);
//             assert_eq!(left_top.x, a_p.left);
//             assert_eq!(left_top.y, a_p.top);
//             assert_eq!(right_bottom.x, a_p.right);
//             assert_eq!(right_bottom.y, a_p.bottom);
//         }
//     }

//     fn asset_quad(query: Query<(Entity, &Quad, &AbsolutePosition)>) {
//         // log::warn!("asset_quad======");
//         for (_e, quad, a_p) in query.iter() {
//             // println!("e: {:?}, quad: {:?}, a_p:{:?}", _e, quad, a_p);
//             assert_eq!(quad.mins.x, a_p.left);
//             assert_eq!(quad.mins.y, a_p.top);
//             assert_eq!(quad.maxs.x, a_p.right);
//             assert_eq!(quad.maxs.y, a_p.bottom);
//         }
//     }
// }

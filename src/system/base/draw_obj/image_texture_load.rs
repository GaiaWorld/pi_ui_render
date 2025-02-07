use std::{marker::PhantomData, ops::Deref};

use pi_world::{alter::Alter, event::{ComponentAdded, ComponentChanged}, fetch::OrDefault, filter::With, insert::Component, prelude::{Changed, ComponentRemoved, Entity, Has, ParamSet, Query, SingleRes}, single_res::SingleResMut, world::FromWorld};
use pi_bevy_ecs_extend::prelude::{OrInitSingleRes, OrInitSingleResMut};

use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{asimage_url::{self, RenderTarget}, render_cross::GraphId, NodeId, PiRenderDevice, PiRenderGraph, PiRenderQueue, TextureKeyAlloter};
use pi_hal::{loader::AsyncLoader, runtime::RENDER_RUNTIME};
use pi_null::Null;
use pi_render::rhi::asset::{AssetWithId, TextureAssetDesc, TextureRes};
use pi_share::Share;
use smallvec::SmallVec;
use pi_world::prelude::Plugin;
use crate::{prelude::UiStage, system::base::pass::pass_life};
use pi_world::prelude::IntoSystemConfigs;


use crate::{components::{calc::{InPassId, Texture}, pass_2d::{Camera, ParentPassId}}, resource::{GlobalDirtyMark, IsRun, OtherDirtyType, RenderObjType}, system::base::pass::update_graph::{self, get_to, AsImageRefCount}};


pub struct ImageLoadPlugin;

impl Plugin for ImageLoadPlugin {
    fn build(&self, app: &mut pi_world::prelude::App) {
		app
			// .add_frame_event::<ComponentEvent<Changed<BackgroundImageTexture>>>()
			.add_system(UiStage, add_as_image_graph_depend
				// .in_set(UiSystemSet::NextSetting)

				.after(update_graph::update_graph)
                .before(pass_life::calc_pass_toop_sort)
            )
        ;
    }
}

#[derive(Clone)]
pub struct ImageAwait<Key: 'static + Send + Sync, T>(
    pub Share<SegQueue<(Key, Atom, Handle<AssetWithId<TextureRes>>)>>,
    pub (Vec<(Entity, Atom)>, Vec<(Entity, Atom)>), // 需要在下一帧重新获取Target类型的url对应的Target
    PhantomData<T>
);

impl<Key: 'static + Send + Sync, T> Default for ImageAwait<Key, T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), (Vec::new(), Vec::new()), PhantomData) }
}

pub struct CalcImageLoad<S: std::ops::Deref<Target = Atom>, D: From<Handle<AssetWithId<TextureRes>>>>(PhantomData<(S, D)>);


// 添加asImage资源的图依赖关系
pub fn add_as_image_graph_depend(
    mut query_with_as_image: Query<(&mut AsImageBindList, &InPassId)>,
    query_pass: Query<(&ParentPassId, &GraphId), With<Camera>>,
    as_image_url_changed: ComponentChanged<AsImageBindList>,
    as_image_url_added: ComponentAdded<AsImageBindList>,
    mut rg: SingleResMut<PiRenderGraph>,
    mut ref_count: OrInitSingleResMut<AsImageRefCount>,
) { 
    // log::warn!("add_as_image_graph_depend================{:?}", as_image_url_changed.len());
    if as_image_url_changed.len() == 0 && as_image_url_added.len() == 0 {
        return;
    }
    
    let ref_count = &mut *ref_count;

    for entity in as_image_url_changed.iter().chain(as_image_url_added.iter()) {
        log::warn!("entity================{:?}", entity);
        if let Ok((mut as_image_bind_list, inpass)) = query_with_as_image.get_mut(*entity) {
            let to = get_to(*inpass.0, &query_pass);
            let as_image_bind_list = as_image_bind_list.bypass_change_detection();
            for as_image_bind in as_image_bind_list.0.iter_mut() {
                if as_image_bind.old_before_graph_id != as_image_bind.before_graph_id {
                    if !as_image_bind.old_before_graph_id.is_null() {
                        let c = if let Some(ref_count) = ref_count.release_one((as_image_bind.old_before_graph_id.0.clone(), to)) {
                            ref_count
                        } else {
                            0
                        };
                        if c == 0 { // 引用计数减到0， 删除依赖
                            let _ = rg.remove_depend(as_image_bind.old_before_graph_id.0.clone(), to);
                        }
                    }
                    ref_count.add_one((as_image_bind.before_graph_id.0.clone(), to));
                    log::warn!("add depend: {:?} -> {:?}", as_image_bind.before_graph_id.0.clone(), to);
                    let _ = rg.add_depend(as_image_bind.before_graph_id.0.clone(), to);
                    as_image_bind.old_before_graph_id = as_image_bind.before_graph_id.clone();
                    as_image_bind.after_graph = to;
                    
                }
            }
        }
    }
}


// asimage依赖关系
#[derive(Debug, Component, PartialEq, Eq)]
pub struct AsImageBind {
    pub before_entity: Entity, // 绑定实体
    pub before_graph_id: GraphId, /*新的graphId */
    pub old_before_graph_id: GraphId, /*旧的graphid*/
    pub obj_type: RenderObjType, // 渲染类型(背景图片， border图片， mask图片)
    pub after_graph: NodeId, // 后续绑定图节点
}

// 一个节点可能存在多个引用关系， 所以是数组
// SmallVec<AsImageBind>中不存在重复的引用关系
#[derive(Debug, Component, Default)]
pub struct AsImageBindList(pub SmallVec<[AsImageBind; 1]>);

impl AsImageBindList {
    pub fn push(&mut self, e: AsImageBind) { 
        for i in self.0.iter() {
            if *i == e {
                return;
            }
        }
        self.0.push(e);
    }

    pub fn del(&mut self, e: (Entity, RenderObjType)) -> Option<AsImageBind> { 
        if let Some(i) = self.0.iter().position(|r| {r.before_entity == e.0 && r.obj_type == e.1}) {
            if i == self.0.len() - 1 {
                self.0.pop()
            } else {
                Some(self.0.swap_remove(i))
            }
        } else {
            None
        }
    }
}

/// Image创建，加载对应的图片
/// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
/// 因为BorderImageTexture未加锁，其他线程可能正在使用
/// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
pub fn image_load<
    S: std::ops::Deref<Target = Atom> + From<Atom> + std::cmp::PartialEq + Send + Sync,
    D: From<Texture> + std::ops::Deref<Target=Option<Texture>> + Null + Eq + PartialEq,
    const DIRTY_TYPE: OtherDirtyType,
    T: Deref<Target = RenderObjType> + Send + Sync + 'static + FromWorld,
>(
    query: Query<(Entity, &S), Changed<S>>,
    query_src: Query<(Entity, &S)>,
    query_render_target: Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>,
    // mut del: RemovedComponents<S>,
    texture_assets_mgr: SingleRes<ShareAssetMgr<AssetWithId<TextureRes>>>,
    mut image_await: OrInitSingleResMut<ImageAwait<Entity, S>>,
    queue: SingleRes<PiRenderQueue>,
    device: SingleRes<PiRenderDevice>,
	key_alloter: OrInitSingleRes<TextureKeyAlloter>,

    // mut commands: Commands,
    mut query_set: ParamSet< (Query<(&mut D, Has<S>)>, Query<&mut D>)>,
    mut query_as_image:  Alter<Option<&mut AsImageBindList>, (), AsImageBindList, ()>,
    src_ty: OrInitSingleRes<T>,
    removed: ComponentRemoved<S>,
    mut global_mark: SingleResMut<GlobalDirtyMark>,

	r: OrInitSingleRes<IsRun>,
) {
	if r.0 {
		return;
	}
    let del = &mut query_set.p0();
    for i in removed.iter() {
        // 图片删除，则删除对应的Texture
        if let Ok((mut r, has_s)) = del.get_mut(*i) {
            if !has_s {
                *r = D::null();
            }
        }
    }
    

    let mut f = |d: &mut D, s, _entity| {
		let is_null = d.is_null();
		*d = s;
		is_null
    };

    // 处理图片路径修改，尝试加载图片（异步加载，加载完成后，放入image_await中）
    for (entity, key) in query.iter() {
        load_image::<DIRTY_TYPE, _, _, _>(
            entity,
            key,
            ***src_ty,
            &mut image_await,
            &device,
            &queue,
            query_set.p1(),
            &mut query_as_image,
            &query_render_target,
            &texture_assets_mgr,
			&key_alloter,
            &mut f,
            &mut global_mark,
        );
    }

    set_texture::<DIRTY_TYPE, _, _, _>(***src_ty, &mut image_await, &query_src, query_set.p1(),  &mut query_as_image, &query_render_target, f, &mut global_mark);
	// if is_change {
	// 	for mut r in dirty.iter_mut() {
	// 		**r = true;
	// 	}
	// }
}

#[inline]
pub fn load_image<'w, const DIRTY_TYPE: OtherDirtyType, S: 'static + Send + Sync, D: Eq + PartialEq +  From<Texture> + std::ops::Deref<Target=Option<Texture>>  + Null, F: FnMut(&mut D, D, Entity) -> bool>(
    entity: Entity,
    key: &Atom,
    src_ty: RenderObjType,
    image_await: &mut ImageAwait<Entity, S>,
    device: &PiRenderDevice,
    queue: &PiRenderQueue,
    query_dst: &mut Query<&mut D>,
    query_as_image: &mut Alter<Option<&mut AsImageBindList>, (), AsImageBindList, ()>,
    query_render_target: &Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>,
    texture_assets_mgr: &ShareAssetMgr<AssetWithId<TextureRes>>,
	key_alloter: &TextureKeyAlloter,
    f: &mut F,
    global_mark: &mut GlobalDirtyMark,
) {
    match asimage_url::load_from_asimage_url(key.as_str(), query_render_target) {
        Ok(r) => {
            log::warn!("load image from asimage_url=============");
            match r {
                Some((safe_target_view, graph_id, from_target)) => if let Ok(mut dst) = query_dst.get_mut(entity) {
                    if let Ok(Option::Some(mut as_image)) =  query_as_image.get_mut(entity) {
                        let a = as_image.bypass_change_detection();
                        let (old_graph_id, after_graph) = if let Some(r) = a.del((from_target, src_ty)) {
                            (r.old_before_graph_id.clone(), r.after_graph.clone())
                        } else {
                            (GraphId::default(), Default::default())
                        };
                        a.push(AsImageBind {
                            before_entity: from_target,
                            before_graph_id: graph_id.clone(),
                            old_before_graph_id: old_graph_id.clone(),
                            obj_type: src_ty,
                            after_graph,
                        });
                        log::warn!("image1============{:?}", (&graph_id, &old_graph_id));
                        if graph_id != old_graph_id {
                            // 如果新旧绑定不相等， 需要设置脏标记
                            as_image.set_changed();
                        }
                        
                    } else {
                        let mut r = AsImageBindList::default();
                        r.push(AsImageBind {
                            before_entity: from_target,
                            before_graph_id: graph_id.clone(),
                            old_before_graph_id: GraphId::default(),
                            after_graph: Default::default(),
                            obj_type: src_ty,
                        });
                        let _ = query_as_image.alter(entity, r);     
                    }
                    let r = D::from(Texture::Part(safe_target_view, from_target));
                    if *dst != r {
                        (*f)(&mut dst, r, entity);
                        global_mark.mark.set(DIRTY_TYPE as usize, true);
                    }
                      
                }
                None => image_await.1.0.push((entity, key.clone())),
            };
            return;
        },
        Err(asimage_url::LoadError::MismatchProtocol) => (),
        _r => {log::warn!("load image from asimage_url fail============={:?}", key.as_str());return},
       
    };
    let result = AssetMgr::load(&texture_assets_mgr, &(key.str_hash() as u64));
    match result {
        LoadResult::Ok(r) => {
            if let Ok(mut dst) = query_dst.get_mut(entity) {
				let r = D::from(Texture::All(r));
				if *dst != r {
                    
                    // log::warn!("image catch: {:?}, {:?}", entity, key);
					(*f)(&mut dst, r, entity);
                    global_mark.mark.set(DIRTY_TYPE as usize, true);
				}
                
            }
        }
        _ => {
            let (awaits, device, queue) = (image_await.0.clone(), (*device).clone(), (*queue).clone());
            let (id, key) = (entity, (*key).clone());
            log::debug!("image await: {:?}, {:?}", id, key);
          
			let key_alloter = key_alloter.0.clone();
            RENDER_RUNTIME
                .spawn(async move {
                    let desc = TextureAssetDesc {
                        url: &key,
                        device: &device,
                        queue: &queue,
                        alloter: &key_alloter,
                    };

                    let r = AssetWithId::<TextureRes>::async_load(desc, result).await;
                    match r {
                        Ok(r) => {
                            awaits.push((id, key.clone(), r));
                        }
                        Err(e) => {
                            log::error!("load image fail, {:?}", e);
                        }
                    };
                })
                .unwrap();
        }
    }
}

// 设置纹理， 返回是否修改问题（同一节点，修改图片路径， 且新旧图片尺寸不一致，新图片异步加载会导致脏区域计算问题，此时此时直接设置全局脏）
#[inline]
pub fn set_texture<'w, const DIRTY_TYPE: OtherDirtyType, S: From<Atom> + std::cmp::PartialEq, D: Eq + PartialEq +  From<Texture> + std::ops::Deref<Target=Option<Texture>>  + Null, F: FnMut(&mut D, D, Entity) -> bool>(
    src_ty: RenderObjType,
    image_await: &mut ImageAwait<Entity, S>,
    query_src: &Query<(Entity, &S)>,
    query_dst: &mut Query<&mut D>,
    query_as_image: &mut Alter<Option<&mut AsImageBindList>, (), AsImageBindList, ()>,
    query_render_target: &Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>,
    mut f: F,
    global_mark: &mut GlobalDirtyMark,
) -> bool {
	let mut is_change = false;
    // 处理已经成功加载的图片，放入到对应组件中
    while let Some((entity, key, texture)) = image_await.0.pop() {
        match query_src.get(entity) {
            Ok((_, img)) => {
                // image已经修改，不需要设置texture
                if img != &S::from(key.clone()) {
                    continue;
                }
                if let Ok(mut dst) = query_dst.get_mut(entity) {               
                    // log::warn!("texture_load success 2: {:?}, {:?}, {:?}", id, key, texture.id);
                    is_change =  f(&mut dst, D::from(Texture::All(texture)), entity) || is_change;
                    global_mark.mark.set(DIRTY_TYPE as usize, true);
                }
            }
            // 节点已经销毁，或image已经被删除，不需要设置texture
            _ => continue,
        };
    }

    if image_await.1.0.len() > 0 {
        std::mem::swap(&mut image_await.1.0, &mut image_await.1.1);
        for (entity, key) in image_await.1.1.drain(..) {
            if let Ok((_, img)) =  query_src.get(entity) {
                // image已经修改，不需要设置texture
                if img != &S::from(key.clone()) {
                    continue;
                }

                match asimage_url::load_from_asimage_url(key.as_str(), query_render_target) {
                    Ok(r) => {
                        match r {
                            Some((safe_target_view, graph_id, from_target)) => if let Ok(mut dst) = query_dst.get_mut(entity) {   
                                if let Ok(Option::Some(mut as_image)) =  query_as_image.get_mut(entity) {
                                    let a = as_image.bypass_change_detection();
                                    let (old_graph_id, after_graph) = if let Some(r) = a.del((from_target, src_ty)) {
                                        (r.old_before_graph_id.clone(), r.after_graph.clone())
                                    } else {
                                        (GraphId::default(), Default::default())
                                    };
                                    a.push(AsImageBind {
                                        before_entity: from_target,
                                        before_graph_id: graph_id.clone(),
                                        old_before_graph_id: old_graph_id.clone(),
                                        obj_type: src_ty,
                                        after_graph,
                                    });
                                    log::warn!("image2============{:?}", (&graph_id, &old_graph_id));
                                    if graph_id != old_graph_id {
                                        // 如果新旧绑定不相等， 需要设置脏标记
                                        as_image.set_changed();
                                    }
                                    
                                } else {
                                    let mut r = AsImageBindList::default();
                                    r.push(AsImageBind {
                                        before_entity: from_target,
                                        before_graph_id: graph_id.clone(),
                                        old_before_graph_id: GraphId::default(),
                                        after_graph: Default::default(),
                                        obj_type: src_ty,
                                    });
                                    let _ = query_as_image.alter(entity, r);     
                                }

                                // log::warn!("texture_load success 2: {:?}, {:?}, {:?}", id, key, texture.id);
                                is_change =  f(&mut dst, D::from(Texture::Part(safe_target_view, from_target)), entity) || is_change;
                                global_mark.mark.set(DIRTY_TYPE as usize, true);
                            },
                            None => image_await.1.0.push((entity, key.clone())),
                        };
                    },
                    _ => continue,
                   
                };
            }
            
            
        }
    }
   
	is_change
}

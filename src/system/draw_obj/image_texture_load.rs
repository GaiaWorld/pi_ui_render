use std::marker::PhantomData;

use pi_world::prelude::{Changed, SingleRes, ParamSet, Query, Entity, Has, ComponentRemoved};
use pi_bevy_ecs_extend::prelude::OrInitSingleRes;

use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue, TextureKeyAlloter};
use pi_hal::{loader::AsyncLoader, runtime::RENDER_RUNTIME};
use pi_null::Null;
use pi_render::rhi::asset::{TextureRes, AssetWithId, TextureAssetDesc};
use pi_share::Share;

use crate::components::user::RenderDirty;

use super::calc_text::IsRun;

#[derive(Clone)]
pub struct ImageAwait<Key: 'static + Send + Sync, T>(pub Share<SegQueue<(Key, Atom, Handle<AssetWithId<TextureRes>>)>>, PhantomData<T>);

impl<Key: 'static + Send + Sync, T> Default for ImageAwait<Key, T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), PhantomData) }
}

pub struct CalcImageLoad<S: std::ops::Deref<Target = Atom>, D: From<Handle<AssetWithId<TextureRes>>>>(PhantomData<(S, D)>);

/// Image创建，加载对应的图片
/// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
/// 因为BorderImageTexture未加锁，其他线程可能正在使用
/// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
pub fn image_load<
    S: std::ops::Deref<Target = Atom> + From<Atom> + std::cmp::PartialEq + Send + Sync,
    D: From<Handle<AssetWithId<TextureRes>>> + Null + Eq + PartialEq,
>(
    query: Query<(Entity, &S), Changed<S>>,
    query_src: Query<(Entity, &S)>,
    // mut del: RemovedComponents<S>,
    texture_assets_mgr: SingleRes<ShareAssetMgr<AssetWithId<TextureRes>>>,
    image_await: OrInitSingleRes<ImageAwait<Entity, S>>,
    queue: SingleRes<PiRenderQueue>,
    device: SingleRes<PiRenderDevice>,
	key_alloter: OrInitSingleRes<TextureKeyAlloter>,

    // mut commands: Commands,
    mut query_set: ParamSet< (Query<(&mut D, Has<S>)>, Query<&mut D>)>,
    removed: ComponentRemoved<S>,

	r: OrInitSingleRes<IsRun>,
	mut dirty: Query<&mut RenderDirty>
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
    

    let f = |d: &mut D, s, _entity| {
		let is_null = d.is_null();
		*d = s;
		is_null
    };

    // 处理图片路径修改，尝试加载图片（异步加载，加载完成后，放入image_await中）
    for (entity, key) in query.iter() {
        load_image(
            entity,
            key,
            &image_await,
            &device,
            &queue,
            query_set.p1(),
            &texture_assets_mgr,
			&key_alloter,
            f,
        );
    }

    let is_change = set_texture(&image_await, &query_src, query_set.p1(), f);
	if is_change {
		for mut r in dirty.iter_mut() {
			**r = true;
		}
	}
}

#[inline]
pub fn load_image<'w, S: 'static + Send + Sync, D: Eq + PartialEq + From<Handle<AssetWithId<TextureRes>>> + Null, F: FnMut(&mut D, D, Entity) -> bool>(
    entity: Entity,
    key: &Atom,
    image_await: &ImageAwait<Entity, S>,
    device: &PiRenderDevice,
    queue: &PiRenderQueue,
    query_dst: &mut Query<&'w mut D>,
    texture_assets_mgr: &ShareAssetMgr<AssetWithId<TextureRes>>,
	key_alloter: &TextureKeyAlloter,
    mut f: F,
) {
    let result = AssetMgr::load(&texture_assets_mgr, &(key.str_hash() as u64));
    match result {
        LoadResult::Ok(r) => {
            if let Ok(mut dst) = query_dst.get_mut(entity) {
				let r = D::from(r);
				if *dst != r {
					f(&mut dst, r, entity);
				}
                
            }
        }
        _ => {
            let (awaits, device, queue) = (image_await.0.clone(), (*device).clone(), (*queue).clone());
            let (id, key) = (entity, (*key).clone());

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
pub fn set_texture<'w, S: From<Atom> + std::cmp::PartialEq, D: Eq + PartialEq + From<Handle<AssetWithId<TextureRes>>> + Null, F: FnMut(&mut D, D, Entity) -> bool>(
    image_await: &ImageAwait<Entity, S>,
    query_src: &Query<(Entity, &S)>,
    query_dst: &mut Query<&'w mut D>,
    mut f: F,
) -> bool {
	let mut is_change = false;
    // 处理已经成功加载的图片，放入到对应组件中
    while let Some((id, key, texture)) = image_await.0.pop() {
        match query_src.get(id) {
            Ok((_, img)) => {
                // image已经修改，不需要设置texture
                if img != &S::from(key.clone()) {
                    continue;
                }
                if let Ok(mut dst) = query_dst.get_mut(id) {
                    is_change =  f(&mut dst, D::from(texture), id) || is_change;
                }
            }
            // 节点已经销毁，或image已经被删除，不需要设置texture
            _ => continue,
        };
    }
	is_change
}

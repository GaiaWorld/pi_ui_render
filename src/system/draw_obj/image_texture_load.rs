use std::marker::PhantomData;

use bevy::ecs::{
    prelude::{Component, Entity, EventWriter, RemovedComponents},
    query::Changed,
    system::{Query, Res, Resource},
};
use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
    mgr::{AssetMgr, LoadResult},
};
use pi_async::prelude::AsyncRuntime;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_hal::{loader::AsyncLoader, runtime::MULTI_MEDIA_RUNTIME};
use pi_null::Null;
use pi_render::rhi::asset::{ImageTextureDesc, TextureRes};
use pi_share::Share;

#[derive(Clone, DerefMut, Deref, Resource)]
pub struct ImageAwait<T>(Share<SegQueue<(Entity, Atom, Handle<TextureRes>)>>, PhantomData<T>);

impl<T> Default for ImageAwait<T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), PhantomData) }
}

pub struct CalcImageLoad<S: std::ops::Deref<Target = Atom>, D: From<Handle<TextureRes>>>(PhantomData<(S, D)>);

/// Image创建，加载对应的图片
/// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
/// 因为BorderImageTexture未加锁，其他线程可能正在使用
/// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
pub fn image_change<S: Component + std::ops::Deref<Target = Atom>, D: Component + From<Handle<TextureRes>> + Null>(
    query: Query<(Entity, &S), Changed<S>>,
    query1: Query<(Entity, &S)>,
    mut del: RemovedComponents<S>,
    texture_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    image_await: OrInitRes<ImageAwait<S>>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,

    // mut commands: Commands,
    mut query_dst: Query<&mut D>,
    mut event_writer: EventWriter<ComponentEvent<Changed<D>>>,
) {
    // 图片删除，则删除对应的Texture
    for del in del.iter() {
        if let Ok(mut r) = query_dst.get_mut(del) {
            *r = D::null();
        };
    }

    // let mut insert = Vec::new();

    // 处理图片路径修改，尝试加载图片（异步加载，加载完成后，放入image_await中）
    for (entity, key) in query.iter() {
        let result = AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64));
        match result {
            LoadResult::Ok(r) => {
                if let Ok(mut dst) = query_dst.get_mut(entity) {
                    *dst = D::from(r);
                    event_writer.send(ComponentEvent::new(entity));
                }
            }
            _ => {
                let (awaits, device, queue) = ((*image_await).clone(), (*device).clone(), (*queue).clone());
                let (id, key) = (entity, (*key).clone());

                MULTI_MEDIA_RUNTIME
                    .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                        let desc = ImageTextureDesc {
                            url: &key,
                            device: &device,
                            queue: &queue,
                        };

                        let r = TextureRes::async_load(desc, result).await;
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
        };
    }

    // 处理已经成功加载的图片，放入到对应组件中
    while let Some((id, key, texture)) = image_await.0.pop() {
        match query1.get(id) {
            Ok((_, img)) => {
                // image已经修改，不需要设置texture
                if **img != key {
                    continue;
                }

                // log::info!("load success====================:{:?}", key);
                // insert.push((id, D::from(texture)));
                if let Ok(mut dst) = query_dst.get_mut(id) {
                    *dst = D::from(texture);
                    event_writer.send(ComponentEvent::new(id));
                }
            }
            // 节点已经销毁，或image已经被删除，不需要设置texture
            _ => continue,
        };
    }

    // if insert.len() > 0 {
    // 	#[cfg(feature="trace")]
    // 	let count = insert.len();
    // 	#[cfg(feature="trace")]
    // 	let _ss = tracing::info_span!("image load", count).entered();

    //     commands.insert_or_spawn_batch(insert.into_iter());
    // }
}

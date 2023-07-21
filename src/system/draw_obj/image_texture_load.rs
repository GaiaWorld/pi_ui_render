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
use pi_async_rt::prelude::AsyncRuntime;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_ecs_extend::system_param::res::OrInitRes;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_hal::{loader::AsyncLoader, runtime::MULTI_MEDIA_RUNTIME};
use pi_null::Null;
use pi_render::rhi::asset::{ImageTextureDesc, TextureRes};
use pi_share::Share;

#[derive(Clone, Resource)]
pub struct ImageAwait<T>(Share<SegQueue<(Entity, Atom, Handle<TextureRes>)>>, PhantomData<T>);

impl<T> Default for ImageAwait<T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), PhantomData) }
}

pub struct CalcImageLoad<S: std::ops::Deref<Target = Atom>, D: From<Handle<TextureRes>>>(PhantomData<(S, D)>);

/// Image创建，加载对应的图片
/// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
/// 因为BorderImageTexture未加锁，其他线程可能正在使用
/// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
pub fn image_change<S: Component + std::ops::Deref<Target = Atom> + From<Atom> + std::cmp::PartialEq, D: Component + From<Handle<TextureRes>> + Null>(
    query: Query<(Entity, &S), Changed<S>>,
    query_src: Query<(Entity, &S)>,
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

	let f = |d: &mut D, s, _entity| {
		*d = D::from(s);
	};

    // 处理图片路径修改，尝试加载图片（异步加载，加载完成后，放入image_await中）
    for (entity, key) in query.iter() {
        load_image(entity, key, &image_await, &device, &queue, Some(&mut event_writer), &mut query_dst, &texture_assets_mgr, f);
    }

    set_texture(&image_await, Some(&mut event_writer), &query_src, &mut query_dst, f);
}

#[inline]
pub fn load_image<'w, S: Component, D: Component, F: FnMut(&mut D, Handle<TextureRes>, Entity)>(
    entity: Entity,
	key: &Atom,
    image_await: &ImageAwait<S>,
	device: &PiRenderDevice,
    queue: &PiRenderQueue,
    event_writer: Option<&mut EventWriter<ComponentEvent<Changed<D>>>>,
	query_dst:&mut Query<&'w mut D>,
	texture_assets_mgr: &ShareAssetMgr<TextureRes>,
	mut f: F,
) {
	let result = AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64));
	match result {
		LoadResult::Ok(r) => {
			if let Ok(mut dst) = query_dst.get_mut(entity) {
				f(&mut dst, r, entity);
				if let Some(event_writer) = event_writer {
					event_writer.send(ComponentEvent::new(entity));
				}
				
			}
		}
		_ => {
			let (awaits, device, queue) = (image_await.0.clone(), (*device).clone(), (*queue).clone());
			let (id, key) = (entity, (*key).clone());

			MULTI_MEDIA_RUNTIME
				.spawn(async move {
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
	}
}

#[inline]
pub fn set_texture<'w, S: Component + From<Atom> + std::cmp::PartialEq, D: Component, F: FnMut(&mut D, Handle<TextureRes>, Entity)>(
    image_await: &ImageAwait<S>,
    mut event_writer: Option<&mut EventWriter<ComponentEvent<Changed<D>>>>,
	query_src: &Query<(Entity, &S)>,
	query_dst:&mut Query<&'w mut D>,
	mut f: F,
) {
    // 处理已经成功加载的图片，放入到对应组件中
    while let Some((id, key, texture)) = image_await.0.pop() {
        match query_src.get(id) {
            Ok((_, img)) => {
                // image已经修改，不需要设置texture
                if img != &S::from(key) {
                    continue;
                }
                if let Ok(mut dst) = query_dst.get_mut(id) {
					f(&mut dst, texture, id);
					if let Some(event_writer) = &mut event_writer {
						event_writer.send(ComponentEvent::new(id));
					}
                }
            }
            // 节点已经销毁，或image已经被删除，不需要设置texture
            _ => continue,
        };
    }
}

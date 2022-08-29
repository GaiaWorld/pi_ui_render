use std::marker::PhantomData;

use pi_assets::{asset::Handle, mgr::{AssetMgr, LoadResult}};
use pi_async::rt::AsyncRuntime;
use pi_atom::Atom;
use pi_ecs::{entity::Id, monitor::Event, prelude::{Query, Write, Res, ResMut}};
use pi_ecs_macros::{setup, listen};
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, loader::AsyncLoader};
use pi_render::rhi::{asset::{TextureRes, ImageTextureDesc}, RenderQueue, device::RenderDevice};
use pi_share::{Share};
use crossbeam::queue::SegQueue;

use crate::components::user::Node;

#[derive(Clone, DerefMut, Deref)]
pub struct ImageAwait<T>(Share<SegQueue<(Id<Node>, Atom, Handle<TextureRes>)>>, PhantomData<T>);

impl<T> Default for ImageAwait<T> {
    fn default() -> Self {
        Self(Share::new(SegQueue::new()), PhantomData)
    }
}

pub struct CalcImageLoad<S: std::ops::Deref<Target=Atom>, D: From<Handle<TextureRes>>>(PhantomData<(S, D)>);

#[setup]
impl<S, D> CalcImageLoad<S, D>
where S: std::ops::Deref<Target=Atom> + 'static + Send + Sync , 
	  D: From<Handle<TextureRes>> + 'static + Send + Sync {
	/// Image创建，加载对应的图片
	/// 图片加载是异步，加载成功后，不能立即将图片对应的纹理设置到BorderImageTexture上
	/// 因为BorderImageTexture未加锁，其他线程可能正在使用
	/// 这里是将一个加载成功的Texture放入一个加锁的列表中，在system执行时，再放入到BorderImageTexture中
	#[listen(component=(Node, S, (Create, Modify)))]
	pub fn image_change(
		e: Event,
		mut query: Query<Node, (&S, Write<D>)>,
		texture_assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
		image_await: Res<ImageAwait<S>>,
		queue: Res<RenderQueue>,
		device: Res<RenderDevice>,
	) {
		let (key, mut texture) = query.get_unchecked_mut_by_entity(e.id);
		let result = AssetMgr::load(&texture_assets_mgr, &(key.get_hash() as u64));
		match result {
            LoadResult::Ok(r) => texture.write(D::from(r)),
			_ => {
				let (awaits, device, queue) =( 
					(*image_await).clone(),  
					(*device).clone(), 
					(*queue).clone());
				let (id, key) = (
					unsafe { Id::new(e.id.local())}, 
					(*key).clone());
					
					MULTI_MEDIA_RUNTIME.spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
					let desc = ImageTextureDesc { 
						url: &key,
						device: &device,
						queue: &queue,
					};

					// log::warn!("load image start {:?}", key);
					let r = TextureRes::async_load(desc, result).await;
					match r {
						Ok(r) => {
							// log::warn!("load image ok {:?}", key);
							awaits.push((id, key.clone(), r));
						},
						Err(e) => {
							log::error!("load image fail, {:?}", e);
						},
					};
				}).unwrap();
			}
		}
	}

	// 
	#[system]
	pub fn check_await_texture(
		border_image_await: Res<ImageAwait<S>>,
		mut query: Query<Node, (&S, Write<D>)>,
	) {
		// let awaits = std::mem::replace(&mut border_image_await.0, Share::new(SegQueue::new()));
		let mut r = border_image_await.0.pop();
		while let Some((id, key, texture)) = r {
			r = border_image_await.0.pop();
			
			let mut texture_item = match query.get_mut(id) {
				Some((img, texture_item)) => {
					// image已经修改，不需要设置texture
					if **img != key {
						continue;
					}
					texture_item
				},
				// 节点已经销毁，或image已经被删除，不需要设置texture
				None => continue,
			};
			texture_item.write(D::from(texture));


		}
	}
}
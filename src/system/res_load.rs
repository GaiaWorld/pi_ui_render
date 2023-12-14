use std::{any::Any, sync::Arc};

use bevy_ecs::system::{Resource, Res};
use crossbeam::queue::SegQueue;
use pi_assets::mgr::{AssetMgr, LoadResult};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::system_param::res::OrInitResMut;
use pi_bevy_render_plugin::{PiRenderQueue, PiRenderDevice};
use pi_hal::{runtime::RENDER_RUNTIME, loader::AsyncLoader};
use pi_render::rhi::asset::{TextureRes, ImageTextureDesc};
use pi_share::Share;
use pi_async_rt::prelude::AsyncRuntime;


#[derive(Clone, Resource)]
pub struct ResSuccess {
	pub async_list: Share<SegQueue<(Atom, Arc<dyn Any + Send + Sync + 'static>)>>,
	pub sync_list: Vec<(Atom, Arc<dyn Any + Send + Sync + 'static>)>,
}

impl Default for ResSuccess {
    fn default() -> Self { Self {
		async_list: Share::new(SegQueue::new()),
		sync_list: Vec::new(),
	} }
}


/// 资源列表（await_list为等待加载的列表）
#[derive(Debug, Resource, Default)]
pub struct ResList {
	pub await_list: Vec<Atom>,
}


/// 加载资源， 加载成功后会发出成功事件
pub fn load_res(
	mut res_list: OrInitResMut<ResList>,
	mut success_list: OrInitResMut<ResSuccess>,
	queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
	texture_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
) {
	
	let ResList{await_list} = &mut **res_list;
	for path in await_list.drain(..) {

		// 加载纹理
		if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".ktx") || path.ends_with(".ktx2") {
			let result = AssetMgr::load(&texture_assets_mgr, &(path.get_hash() as u64));
			match result {
				LoadResult::Ok(r) => {
					// 加载成功后, 添加到成功队列
					success_list.sync_list.push((path, r));
				},
				_ => {
					let (async_list, device, queue) = (success_list.async_list.clone(), (*device).clone(), (*queue).clone());
					let path = path.clone();

					RENDER_RUNTIME
						.spawn(async move {
							let desc = ImageTextureDesc {
								url: &path,
								device: &device,
								queue: &queue,
							};

							let r = TextureRes::async_load(desc, result).await;
							match r {
								Ok(r) => {
									async_list.push((path.clone(), r));
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


		// 暂时不支持其他类型资源的加载
		
	}
}

// #[inline]
// pub fn load_texture<'w, A: Asset, G: Garbageer<A>>(
//     key: DefaultKey,
//     path: &Atom,
//     image_await: &ImageAwait<DefaultKey, ()>,
//     device: &PiRenderDevice,
//     queue: &PiRenderQueue,
//     texture_assets_mgr: &ShareAssetMgr<TextureRes>,
// 	result: LoadResult<'w, A, G>,
// ) {
//     let (awaits, device, queue) = (image_await.0.clone(), (*device).clone(), (*queue).clone());
// 	let path = path.clone();

// 	RENDER_RUNTIME
// 		.spawn(async move {
// 			let desc = ImageTextureDesc {
// 				url: &path,
// 				device: &device,
// 				queue: &queue,
// 			};

// 			let r = TextureRes::async_load(desc, result).await;
// 			match r {
// 				Ok(r) => {
// 					awaits.push((key, path.clone(), r));
// 				}
// 				Err(e) => {
// 					log::error!("load image fail, {:?}", e);
// 				}
// 			};
// 		})
// 		.unwrap();
// }

use std::any::Any;

use pi_world::prelude::SingleRes;
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use crossbeam::queue::SegQueue;
use pi_assets::mgr::{AssetMgr, LoadResult};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue, TextureKeyAlloter};
use pi_hal::{runtime::RENDER_RUNTIME, loader::AsyncLoader};
use pi_render::rhi::asset::{TextureRes, AssetWithId, TextureAssetDesc};
use pi_share::Share;
use pi_async_rt::prelude::AsyncRuntime;


#[derive(Clone)]
pub struct ResSuccess {
	pub async_list: Share<SegQueue<(Atom, Share<dyn Any + Send + Sync + 'static>)>>,
	pub sync_list: Vec<(Atom, Share<dyn Any + Send + Sync + 'static>)>,
}

impl Default for ResSuccess {
    fn default() -> Self { Self {
		async_list: Share::new(SegQueue::new()),
		sync_list: Vec::new(),
	} }
}


/// 资源列表（await_list为等待加载的列表）
#[derive(Debug, Default)]
pub struct ResList {
	pub await_list: Vec<Atom>,
}


/// 加载资源， 加载成功后会发出成功事件
pub fn load_res(
	mut res_list: OrInitSingleResMut<ResList>,
	mut success_list: OrInitSingleResMut<ResSuccess>,
	queue: SingleRes<PiRenderQueue>,
    device: SingleRes<PiRenderDevice>,
	texture_assets_mgr: SingleRes<ShareAssetMgr<AssetWithId<TextureRes>>>,
	key_alloter: OrInitSingleRes<TextureKeyAlloter>,
) {
	
	let ResList{await_list} = &mut **res_list;
	let key_alloter: Share<pi_key_alloter::KeyAlloter> = (*key_alloter).0.clone();
	for path in await_list.drain(..) {

		// 加载纹理
		if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".ktx") || path.ends_with(".ktx2") {
			let result = AssetMgr::load(&texture_assets_mgr, &(path.str_hash() as u64));
			match result {
				LoadResult::Ok(r) => {
					// 加载成功后, 添加到成功队列
					success_list.sync_list.push((path, r));
				},
				_ => {
					let (async_list, device, queue) = (success_list.async_list.clone(), (*device).clone(), (*queue).clone());
					let path = path.clone();

					let key_alloter = key_alloter.clone();
					RENDER_RUNTIME
						.spawn(async move {
							let desc = TextureAssetDesc {
								alloter: &key_alloter,            
								url: &path,
								device: &device,
								queue: &queue,
							};

							let r = AssetWithId::<TextureRes>::async_load(desc, result).await;
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

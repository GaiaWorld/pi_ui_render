use std::any::Any;

use pi_world::{prelude::SingleRes, world::Entity};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use crossbeam::queue::SegQueue;
use pi_assets::mgr::{AssetMgr, LoadResult};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue, TextureKeyAlloter};
use pi_hal::{runtime::RENDER_RUNTIME, loader::AsyncLoader};
use pi_render::{renderer::texture::{ImageTextureFrame, KeyImageTextureFrame}, rhi::asset::{AssetWithId, ImageTextureDesc, TextureAssetDesc, TextureRes}};
use pi_share::Share;
use pi_async_rt::prelude::AsyncRuntime;

use crate::system::base::draw_obj::image_texture_load::{GuiTextureCombineAtlas2DMgr, ImageAwait};


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

/// 资源列表（await_list为等待加载的列表）
#[derive(Debug, Default)]
pub struct ResList2 {
	pub await_list: Vec<Atom>,
}

/// 加载资源， 加载成功后会发出成功事件
pub fn load_res(
	mut res_list: OrInitSingleResMut<ResList>,
	// mut res_list2: OrInitSingleResMut<ResList2>,
	mut success_list: OrInitSingleResMut<ResSuccess>,
	queue: SingleRes<PiRenderQueue>,
    device: SingleRes<PiRenderDevice>,
	// texture_assets_mgr: SingleRes<ShareAssetMgr<TextureRes>>,
	texture_assets_mgr: SingleRes<ShareAssetMgr<ImageTextureFrame>>,
	mut image_await: OrInitSingleResMut<ImageAwait<Entity, Atom>>,
	key_alloter: OrInitSingleRes<TextureKeyAlloter>,
	mut texture_combine_mgr: OrInitSingleResMut<GuiTextureCombineAtlas2DMgr>,
) {
	
	let ResList{await_list} = &mut **res_list;

	for path in await_list.drain(..) {
		// 加载纹理
		if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".ktx") || path.ends_with(".ktx2") {
			let key = KeyImageTextureFrame {
				url: path.clone(),
				file: true,
				compressed: path.as_str().ends_with(".ktx"),
				cancombine: true,
			};

			let path = path.clone();
			if let Some(tex) = image_await.2.async_load(
				Entity::default(),
				key,
				wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				&*device,
				&*queue,
				&texture_assets_mgr.0,
			){
				pi_hal::texture::remove_res_cache(&path);
				success_list.sync_list.push((path, tex));
			}
					
		} 

		// 暂时不支持其他类型资源的加载
	}
	image_await.2.check_combine(&device, &queue, &mut texture_combine_mgr.0);
	 while let Some((_, key, texture)) = image_await.2.success.pop() {
		let KeyImageTextureFrame{url,..} = key;
		// println!("=========async_list push2: {}", url);
		pi_hal::texture::remove_res_cache(&url);
        success_list.async_list.push((url, texture));
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

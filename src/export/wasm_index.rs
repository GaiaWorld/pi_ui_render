#[allow(unused_unsafe)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn create_engine(gl: WebGlRenderingContext, res_mgr: &ResMgr) -> u32 {
    let use_vao = unsafe { useVao() };
	// let use_vao = false;
    // let gl = WebglHalContext::new(gl, fbo, false);
	let gl = WebglHalContext::new(gl, use_vao);
	let res_mgr = res_mgr.get_inner().clone();
	seting_res_mgr(&mut res_mgr.borrow_mut());
	let engine = Engine::new(gl, res_mgr);
	let r = Box::into_raw(Box::new(UnsafeMut::new(Share::new(engine)))) as u32;
    r
}
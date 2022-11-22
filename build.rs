use std::{borrow::Cow, path::Path, fs::read_to_string};

fn main() {

	println!("cargo:rerun-if-changed=resource/");

	let target = Path::new("src/shaders/");

	let sharder_infos = [
		["color", "resource/color.vert", "resource/color.frag"],
		["image", "resource/image.vert", "resource/image.frag"],
		["text", "resource/text.vert", "resource/text.frag"],
		// ["color", "resource/common.vert", "resource/color.frag"],
		// ["color", "resource/common.vert", "resource/color.frag"],
	];
	
	

	for item in sharder_infos.iter() {
		render_compile::compile_and_out(
			item[0], 
			render_compile::ProcessedShader::Glsl(Cow::Borrowed(read_to_string(item[1]).unwrap().as_str()), naga::ShaderStage::Vertex),
			render_compile::ProcessedShader::Glsl(Cow::Borrowed(read_to_string(item[2]).unwrap().as_str()), naga::ShaderStage::Fragment),
			&target
		);
	}
	
	let mods: Vec<String> = sharder_infos.iter().map(|r| {format!("pub mod {};", r[0])}).collect();
	std::fs::write(target.join("mod.rs"), mods.join("\n")).unwrap();
}
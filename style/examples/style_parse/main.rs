use std::{fs::{DirEntry, read}, path::Path};

use pi_style::{style_parse::parse_class_map_from_string};

fn main() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
	let mut cb = |dwcss: &DirEntry| {
		let file = read(dwcss.path());
		if let Ok(r) = file {
			let file = String::from_utf8(r).unwrap();
			parse_class_map_from_string(file.as_str()).unwrap();
		}
	};
	visit_dirs(&Path::new("examples/style_parse/resource/"), &mut cb).unwrap();

}

pub fn visit_dirs<F: FnMut(&DirEntry)>(path: &Path, cb: &mut F) -> std::io::Result<()> {
	if path.is_dir() {
		for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
	}
	Ok(())
}
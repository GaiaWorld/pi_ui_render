use pi_bevy_render_plugin::Record;
use pi_ui_render::system::base::node::cmd_play::UIRecord;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("gui_cmd/cmd_1784600969895_1.gui_cmd");
    let data = fs::read(path).unwrap();
    let records: Vec<Record> = postcard::from_bytes(&data).unwrap();

    for (fi, record) in records.iter().enumerate() {
        for (key, cmd_data) in &record.cmds {
            if *key != 1 { continue; }
            let ui = match postcard::from_bytes::<UIRecord>(cmd_data) {
                Ok(r) => r,
                Err(_) => continue,
            };

            for s in &ui.style_commands {
                for v in &s.values {
                    let debug_str = format!("{:?}", v);
                    // TransformWillChange 相关的 pattern
                    if debug_str.contains("TransformWillChange") || debug_str.contains("will_change") {
                        println!("Frame {} entity={:?}: {:?}", fi, s.entity, v);
                    }
                }
            }

            for cmd in &ui.other_commands_list {
                let debug_str = format!("{:?}", cmd);
                if debug_str.contains("TransformWillChange") || debug_str.contains("will_change") {
                    println!("Frame {} other_cmd: {:?}", fi, cmd);
                }
            }
        }
    }
    println!("done");
}

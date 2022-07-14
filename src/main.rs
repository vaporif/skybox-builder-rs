use std::env;

use skybox_composer::merge_all_files;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => merge_all_files(""), 
        1 => merge_all_files(&args[0]),
        _ => panic!("Either enter images to merge path or run without args")
    }
}



use std::fs;

pub fn merge_all_files(path: &str) {
    for file in fs::read_dir(path).expect("wrong path") {
        println!("{}", file.unwrap().path().display());
    }
}

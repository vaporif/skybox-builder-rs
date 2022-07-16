use std::{env, io::Error};

use skybox_composer::merge_all_files;

fn main() {
    if let Err(e) = run() {
        panic!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    let _args: Vec<String> = env::args().collect();
    match env::args().count() {
        1 => merge_all_files()?,
        _ => panic!("Params not supported"),
    }

    Ok(())
}

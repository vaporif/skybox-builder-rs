use clap::{command, Arg};
use std::{env, io::Error};

use skybox_composer::process_files;

fn main() {
    if let Err(e) = run() {
        panic!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    let delete_flag = "delete";
    let matches = command!()
        .version("0.2.1")
        .author("Dmytro O. <vaporif@gmail.com>")
        .about("Skybox file merger")
        .arg(
            Arg::new(delete_flag)
                .short('d')
                .action(clap::ArgAction::SetTrue)
                .help("delete input images after the skybox is created"),
        )
        .get_matches();

    let delete_input_files = matches.get_flag(delete_flag);
    process_files(delete_input_files)?;

    Ok(())
}

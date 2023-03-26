use clap::{command, Arg};
use skybox_composer::generation::process_files;
use std::env;

fn main() -> anyhow::Result<()> {
    let delete_flag = "delete";
    let matches = command!()
        .version(env!("CARGO_PKG_VERSION"))
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
    process_files(delete_input_files)
}

use std::path::{Path, PathBuf};

mod utils;

use nom_kconfig::{parse_kconfig, KconfigFile, KconfigInput};

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("u-boot");
    utils::clone_if_not_exists("git@github.com:u-boot/u-boot.git", &destination)?;
    parse_kconfig_files(&destination, destination.join("Kconfig"))?;

    Ok(())
}

fn parse_kconfig_files(root_dir: &Path, entrypoint: PathBuf) -> std::io::Result<()> {
    let cur_kconfig_file = KconfigFile::new(root_dir.to_path_buf(), entrypoint.clone());
    let input = cur_kconfig_file.read_to_string().unwrap();
    let kconfig_parse_result = parse_kconfig(KconfigInput::new_extra(&input, cur_kconfig_file));

    if let Err(e) = kconfig_parse_result {
        panic!(
            "failed to parse kconfig file {:?}, error is {:?}",
            entrypoint, e
        );
    }
    println!("Parsed: {:#?}", kconfig_parse_result.unwrap().1);
    Ok(())
}

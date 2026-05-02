use std::fs;

use nom_kconfig::KconfigFile;

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("openwrt");
    utils::clone_if_not_exists("https://git.openwrt.org/openwrt/openwrt.git", &destination)?;
    let _ = fs::create_dir(destination.join("tmp"));
    let _ = fs::write(destination.join("tmp").join(".config-target.in"), "");

    parsing::parse_kconfig_file(KconfigFile::new(
        destination.clone(),
        destination.join("target").join("Config.in"),
    ))?;

    Ok(())
}

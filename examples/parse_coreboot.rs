use std::fs;

use nom_kconfig::KconfigFile;

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("coreboot");
    utils::clone_if_not_exists("https://review.coreboot.org/coreboot.git", &destination)?;
    let _ = fs::create_dir(destination.join("site-local"));
    let _ = fs::write(destination.join("site-local").join("Kconfig"), "");

    parsing::parse_kconfig_file(KconfigFile::new(
        destination.clone(),
        destination.join("src/Kconfig"),
    ))?;

    Ok(())
}

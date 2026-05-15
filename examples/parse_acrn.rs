use std::collections::HashMap;

use nom_kconfig::KconfigFile;
use tracing::info;

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    info!("Parsing ACRN Kconfig files...");

    let dir = std::env::temp_dir();
    let destination = dir.join("acrn");
    utils::clone_if_not_exists(
        "https://github.com/projectacrn/acrn-kernel.git",
        &destination,
    )?;

    let kconfig_file = KconfigFile::new_with_vars::<String>(
        destination.clone(),
        destination.join("Kconfig"),
        &HashMap::default(),
        &HashMap::default(),
    );

    parsing::parse_kconfig_file(kconfig_file)?;

    Ok(())
}

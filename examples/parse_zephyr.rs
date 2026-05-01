use std::{collections::HashMap, fs};

use nom_kconfig::KconfigFile;

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    if cfg!(not(feature = "kconfiglib")) {
        eprintln!("This example requires the 'kconfiglib' feature to be enabled.");
        eprintln!(
            "Please run the following command:\n cargo run --features kconfiglib --example parse_zephyr {}",
            std::env::args().skip(1).collect::<Vec<_>>().join(" ")
        );
        std::process::exit(1);
    }

    println!("Parsing Zephyr Kconfig files...");

    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("zephyr");
    utils::clone_if_not_exists(
        "https://github.com/zephyrproject-rtos/zephyr.git",
        &destination,
    )?;

    let binary_dir = destination.join("build").join("zephyr");
    let _ = fs::create_dir_all(&binary_dir);

    let _ = fs::write(destination.join("boards").join("Kconfig.v2"), "");
    //let _ = fs::write(
    //    destination.join("drivers/modem/hl78xx/hl78xx_evt_monitor/Kconfig.hl78xx_evt_monitor"),
    //    "",
    //);
    let _ = fs::write(
        destination.join("subsys/logging/Kconfig.template.log_config_inherit"),
        "",
    );

    let kconfig_file = KconfigFile::new_with_vars(
        destination.clone(),
        destination.join("Kconfig"),
        &HashMap::from([
            ("ZEPHYR_BASE", destination.display().to_string().as_str()),
            (
                "KCONFIG_BINARY_DIR",
                binary_dir.display().to_string().as_str(),
            ),
        ]),
        &HashMap::default(),
    );

    parsing::parse_kconfig_file(kconfig_file)?;

    Ok(())
}

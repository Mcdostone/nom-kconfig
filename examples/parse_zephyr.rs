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
    parsing::parse_from_entrypoint(&destination, destination.join("Kconfig"))?;

    Ok(())
}

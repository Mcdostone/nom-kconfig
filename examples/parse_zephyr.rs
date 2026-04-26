mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
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

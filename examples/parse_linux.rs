use std::fs;

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("linux");
    utils::clone_if_not_exists(
        "https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git",
        &destination,
    )?;
    let _ = fs::create_dir(destination.join("tmp"));
    let _ = fs::write(destination.join("tmp").join(".config-target.in"), "");

    parsing::parse_all_kconfig_files(&destination)?;

    Ok(())
}

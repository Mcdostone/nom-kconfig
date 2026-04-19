use std::{fs, process::Command};

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("linux-next");
    utils::clone_if_not_exists(
        "https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git",
        &destination,
    )?;

    Command::new("git")
        .args([
            "remote",
            "add",
            "linux-next",
            "https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git",
        ])
        .current_dir(&destination)
        .output()?;

    Command::new("git")
        .args(["fetch", "linux-next"])
        .current_dir(&destination)
        .output()?;

    Command::new("git")
        .args(["fetch", "--tags", "linux-next"])
        .current_dir(&destination)
        .output()?;

    Command::new("git")
        .args(["checkout", "master"])
        .current_dir(&destination)
        .output()?;

    Command::new("git")
        .args(["remote", "update"])
        .current_dir(&destination)
        .output()?;

    let _ = fs::create_dir(destination.join("tmp"));
    let _ = fs::write(destination.join("tmp").join(".config-target.in"), "");

    parsing::parse_all_kconfig_files(&destination)?;

    Ok(())
}

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use tracing::{info, warn, Level};

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(Level::TRACE)
        .init();
}

pub fn clone(url: &str, destination: &Path) -> std::io::Result<PathBuf> {
    info!(
        "Cloning repository from '{}' to '{}'",
        url,
        destination.display()
    );
    let _output = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            url,
            &destination.display().to_string(),
        ])
        .output()?;

    Ok(destination.to_path_buf())
}

pub fn clone_if_not_exists(url: &str, destination: &Path) -> std::io::Result<PathBuf> {
    if fs::metadata(destination).is_ok() {
        warn!(
            "Directory '{}' already exists, skipping cloning repository",
            destination.display()
        );
    } else {
        clone(url, destination).unwrap();
    }
    Ok(destination.to_path_buf())
}

#[allow(dead_code)]
fn main() {}

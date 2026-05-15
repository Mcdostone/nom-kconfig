use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use tracing::{info, warn, Level};
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE | FmtSpan::ENTER)
        .with_writer(std::io::stderr)
        .with_max_level(Level::DEBUG)
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
    match _output.status.success() {
        true => info!("Successfully cloned repository"),
        false => warn!(
            "Failed to clone repository, git output: {}",
            String::from_utf8_lossy(&_output.stderr)
        ),
    }

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

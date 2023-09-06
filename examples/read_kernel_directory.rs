use nom_kconfig::{kconfig::parse_kconfig, Kconfig, KconfigFile, KconfigInput};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kernel_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("benches")
        .join("linux-6.4.10");

    let entries: Vec<PathBuf> = list_kconfig_files(&kernel_directory);
    let mut kconfigs: Vec<Kconfig> = vec![];
    for current_kconfig in entries.iter() {
        let filename = current_kconfig
            .strip_prefix(&kernel_directory)
            .unwrap_or(current_kconfig);

        let kconfig_file = KconfigFile::new(kernel_directory.to_path_buf(), filename.to_path_buf());
        let input = kconfig_file.read_to_string()?;
        match parse_kconfig(KconfigInput::new_extra(&input, kconfig_file)) {
            Ok((_, kconfig)) => kconfigs.push(kconfig),
            Err(e) => return Err(Box::new(e.map_input(|f| (f.to_string().clone(), f.extra)))),
        }
    }

    println!("{} Kconfig file have been read", kconfigs.len());
    Ok(())
}

/// Returns the list of Kconfig files to parse
pub fn list_kconfig_files(root_dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = vec![];
    if root_dir.metadata().unwrap().is_dir() {
        entries.extend(
            WalkDir::new(root_dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry: &walkdir::DirEntry| entry.file_type().is_file())
                .map(|x| x.path().canonicalize())
                .filter_map(|e: Result<PathBuf, std::io::Error>| e.ok()),
        );
    } else {
        entries.push(root_dir.to_path_buf());
    }

    entries
}

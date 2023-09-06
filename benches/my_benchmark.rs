use std::path::{Path, PathBuf};

use criterion::{criterion_group, criterion_main, Criterion};
use nom_kconfig::{kconfig::parse_kconfig, Kconfig, KconfigFile, KconfigInput};
use walkdir::WalkDir;

fn parse_files(kernel_directory: &str, files: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut kconfigs: Vec<Kconfig> = vec![];
    let root_directory = Path::new(kernel_directory);
    for current_kconfig in files.iter() {
        let kconfig_file = KconfigFile::new(
            root_directory.to_path_buf(),
            root_directory.join(current_kconfig),
        );
        let input = kconfig_file.read_to_string()?;
        match parse_kconfig(KconfigInput::new_extra(&input, kconfig_file)) {
            Ok((_, kconfig)) => kconfigs.push(kconfig),
            Err(e) => return Err(Box::new(e.map_input(|f| (f.to_string().clone(), f.extra)))),
        }
    }
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

const KERNEL_DIRECTORY: &str = "./benches/linux-6.4.10"; //PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                                                         // .join("benches")
                                                         // .join("linux-6.4.10").to_str().unwrap();

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_kconfig_1_config_entry", |b| {
        b.iter(|| {
            let _ = parse_files(KERNEL_DIRECTORY, vec!["drivers/accel/ivpu/Kconfig"]);
        })
    });

    c.bench_function("parse_kconfig_lot_of_sources", |b| {
        b.iter(|| {
            let _ = parse_files(KERNEL_DIRECTORY, vec!["arch/arm/Kconfig"]);
        })
    });

    c.bench_function("parse_kconfig", |b| {
        b.iter(|| {
            let _ = parse_files(KERNEL_DIRECTORY, vec!["Kconfig"]);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

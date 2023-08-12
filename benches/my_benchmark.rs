use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nom_kconfig::{kconfig::parse_kconfig, KconfigFile, KconfigInput};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_kconfig_1_config_entry", |b| {
        b.iter(|| {
            let input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("benches")
                .join("linux-6.4.10")
                .join("drivers/accel/ivpu/Kconfig");
            let kconfig_file: KconfigFile = KconfigFile::new(
                input_file.parent().unwrap().to_path_buf(),
                input_file.clone(),
            );
            let content: String = kconfig_file.read_to_string().unwrap();
            let input: nom_locate::LocatedSpan<&str, KconfigFile> = KconfigInput::new_extra(&content, kconfig_file);
            let _ = parse_kconfig(black_box(input.clone()));
        })
    });



    c.bench_function("parse_kconfig_lot_of_sources", |b| {
        b.iter(|| {
            let input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("benches")
                .join("linux-6.4.10")
                .join("arch/arm/Kconfig");
            let kconfig_file: KconfigFile = KconfigFile::new(
                input_file.parent().unwrap().to_path_buf(),
                input_file.clone(),
            );
            let content: String = kconfig_file.read_to_string().unwrap();
            let input: nom_locate::LocatedSpan<&str, KconfigFile> = KconfigInput::new_extra(&content, kconfig_file);
            let _ = parse_kconfig(black_box(input.clone()));
        })
    });
        
    c.bench_function("parse_kconfig", |b| {
        b.iter(|| {
            let input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("benches")
                .join("linux-6.4.10")
                .join("Kconfig");
            let kconfig_file: KconfigFile = KconfigFile::new(
                input_file.parent().unwrap().to_path_buf(),
                input_file.clone(),
            );
            let content: String = kconfig_file.read_to_string().unwrap();
            let input: nom_locate::LocatedSpan<&str, KconfigFile> = KconfigInput::new_extra(&content, kconfig_file);
            let _ = parse_kconfig(black_box(input.clone()));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

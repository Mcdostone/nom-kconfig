use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nom_kconfig::{kconfig::parse_kconfig, KconfigFile, KconfigInput};

fn criterion_benchmark(c: &mut Criterion) {
    let input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("Kconfig");
    let kconfig_file: KconfigFile = KconfigFile::new(
        input_file.parent().unwrap().to_path_buf(),
        input_file.clone()
    );

    let content = kconfig_file.read_to_string().unwrap();
    let input = KconfigInput::new_extra(&content, kconfig_file);
    c.bench_function("parse_kconfig", |b| {
        b.iter(|| parse_kconfig(black_box(input.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

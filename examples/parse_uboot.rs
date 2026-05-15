use nom_kconfig::KconfigFile;

mod parsing;
mod utils;

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("u-boot");
    utils::clone_if_not_exists("https://github.com/u-boot/u-boot.git", &destination)?;
    parsing::parse_kconfig_file(KconfigFile::new(
        destination.clone(),
        destination.join("Kconfig"),
    ))?;

    Ok(())
}

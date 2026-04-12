use std::{
    fs::{self, File},
    io::Read,
    path::{self, Path, PathBuf},
};

use nom_kconfig::{parse_kconfig, KconfigFile, KconfigInput};
use tracing::{debug, error};

#[allow(dead_code)]
pub fn parse_from_entrypoint(root_dir: &Path, entrypoint: PathBuf) -> std::io::Result<()> {
    let cur_kconfig_file = KconfigFile::new(root_dir.to_path_buf(), entrypoint.clone());
    debug!("Parsing kconfig file: {:?}", cur_kconfig_file);
    let input = cur_kconfig_file.read_to_string().unwrap();
    let kconfig_parse_result = parse_kconfig(KconfigInput::new_extra(&input, cur_kconfig_file));

    if let Err(e) = kconfig_parse_result {
        error!(
            "failed to parse kconfig file {:?}, error is {:?}",
            entrypoint, e
        );
        error!(
            "Please run the following command to debug:\n cargo run --all-features --example parse_file -- --root-dir '{}' '{}'",
            root_dir.display(), entrypoint.display()
        );

        panic!("");
    }
    println!("Parsed: {:#?}", kconfig_parse_result.unwrap().1);
    Ok(())
}

#[allow(dead_code)]
pub fn parse_all_kconfig_files(root_dir: &PathBuf) -> std::io::Result<()> {
    let linux_source = fs::canonicalize(root_dir)?;
    #[allow(clippy::incompatible_msrv)]
    let linux_source = path::absolute(linux_source)?;
    for entry in walkdir::WalkDir::new(&linux_source)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        #[allow(clippy::incompatible_msrv)]
        let path = path::absolute(path)?;

        if path.starts_with(linux_source.join("scripts"))
            || path.starts_with(
                linux_source
                    .join("tools")
                    .join("verification")
                    .join("rvgen")
                    .join("rvgen")
                    .join("templates"),
            )
        {
            continue;
        }

        if path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|ext| ext.eq("Kconfig"))
            .unwrap_or(false)
        {
            eprintln!("Parsing file '{}'", path.display());

            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let path_no_root = path.strip_prefix(&linux_source).unwrap();

            let cur_kconfig_file =
                KconfigFile::new(linux_source.clone(), PathBuf::from(path_no_root));
            let input = cur_kconfig_file.read_to_string().unwrap();
            let kconfig_parse_result =
                parse_kconfig(KconfigInput::new_extra(&input, cur_kconfig_file));

            if let Err(e) = kconfig_parse_result {
                panic!(
                    "failed to parse kconfig file {:?}, error is {:?}",
                    path_no_root, e
                );
            }
            println!("Parsed: {:#?}", kconfig_parse_result.unwrap().1);
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn main() {}

use std::{
    fs::{self, File},
    io::Read,
    path::{self, PathBuf},
};

use nom_kconfig::{parse_kconfig, Entry, Kconfig, KconfigFile, KconfigInput};
use tracing::error;

#[allow(dead_code)]
pub fn parse_kconfig_file(kconfig_file: KconfigFile) -> std::io::Result<()> {
    let input = kconfig_file.read_to_string().unwrap();
    let kconfig_parse_result = parse_kconfig(KconfigInput::new_extra(&input, kconfig_file.clone()));

    if let Err(e) = kconfig_parse_result {
        error!("{}", e);
        error!(
            "Please run the following command to debug:\n cargo run --all-features --example parse_file -- --root-dir '{}' '{}'",
            kconfig_file.root_dir.display(), e.resolved_path.display()
        );

        panic!("");
    }

    let root_kconfig = kconfig_parse_result.unwrap().1;
    println!(
        "{} have been parsed successfully.",
        number_of_parsed_file(&root_kconfig)
    );
    //println!("Parsed: {:#?}", root_kconfig);
    Ok(())
}

#[allow(dead_code)]
pub fn number_of_parsed_file(kconfig: &Kconfig) -> usize {
    let mut count = 1;
    for entry in &kconfig.entries {
        if let Entry::Source(source) = entry {
            for source_kconfig in &source.kconfigs {
                count += number_of_parsed_file(source_kconfig);
            }
        }
    }
    count
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
                panic!("{}", e);
            }
            println!("Parsed: {:#?}", kconfig_parse_result.unwrap().1);
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn main() {}

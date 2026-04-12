use std::{
    fs::{self, File},
    io::Read,
    path::{self, PathBuf},
};

mod utils;

use nom_kconfig::{parse_kconfig, KconfigFile, KconfigInput};

fn main() -> std::io::Result<()> {
    utils::init_tracing();
    let dir = std::env::temp_dir();
    let destination = dir.join("linux");
    utils::clone_if_not_exists(
        "https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git",
        &destination,
    )?;
    let _ = fs::create_dir(destination.join("tmp"));
    let _ = fs::write(destination.join("tmp").join(".config-target.in"), "");

    parse_kconfig_files(&destination)?;

    Ok(())
}

fn parse_kconfig_files(linux_source: &PathBuf) -> std::io::Result<()> {
    let linux_source = fs::canonicalize(linux_source)?;
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

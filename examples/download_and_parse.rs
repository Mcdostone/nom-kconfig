use std::{
    fs::{self, File},
    io::Read,
    path::{self, PathBuf},
};

use nom_kconfig::{parse_kconfig, KconfigFile, KconfigInput};

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <linux_version>", args[0]);
        eprintln!("Example: {} 6.18.1", args[0]);
        std::process::exit(1);
    }

    let mut version = args[1].to_string();
    if version.ends_with(".0") {
        version = version.replace(".0", "");
    }
    let linux_dir = download_and_extract_linux(&version)?;
    parse_kconfig_files(&linux_dir)?;

    Ok(())
}

fn download_and_extract_linux(version: &str) -> std::io::Result<PathBuf> {
    let linux_dir = format!("linux-{}", version);
    let linux_path: PathBuf = PathBuf::from(&linux_dir);

    // Check if already extracted
    if linux_path.exists() {
        println!("Linux kernel {} already exists, skipping download", version);
        return Ok(linux_path);
    }

    let tarball = format!("linux-{}.tar.xz", version);
    let url = format!("https://cdn.kernel.org/pub/linux/kernel/v6.x/{}", tarball);

    println!("Downloading Linux kernel {} from {}", version, url);

    // Download the tarball
    let response = reqwest::blocking::get(&url)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    if !response.status().is_success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to download: HTTP {}", response.status()),
        ));
    }

    let bytes = response
        .bytes()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("Downloaded {} bytes, writing to {}", bytes.len(), tarball);
    std::fs::write(&tarball, bytes)?;

    println!("Extracting {}...", tarball);

    // Extract using tar command
    let output = std::process::Command::new("tar")
        .args(["xf", &tarball])
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to extract tarball: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }
    println!("Successfully extracted to {}", linux_dir);
    std::fs::remove_file(&tarball)?;

    Ok(linux_path)
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

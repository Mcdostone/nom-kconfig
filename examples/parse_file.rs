use std::{env, path::PathBuf};

use nom_kconfig::{parse_kconfig, KconfigFile, KconfigInput};

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <file", args[0]);
        eprintln!("Example: {} linux-6.18/arch/Kconfig", args[0]);
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);

    let kconfig_file = KconfigFile::new(env::current_dir().unwrap(), path.clone());
    let input = kconfig_file.read_to_string().unwrap();
    let parsing_result = parse_kconfig(KconfigInput::new_extra(&input, kconfig_file));
    if let Err(e) = parsing_result {
        panic!("failed to parse kconfig file {:?}, error is {:?}", path, e);
    }
    println!("Parsed: {:#?}", parsing_result.unwrap().1);

    Ok(())
}

use nom_kconfig::{kconfig::parse_kconfig, KconfigFile, KconfigInput};
use std::path::PathBuf;

const INPUT: &str = r#"
# This is a comment

config 64BIT
	bool

config 32BIT
	bool

config GCC_SUPPORTS_DYNAMIC_FTRACE
	def_bool CC_IS_GCC
	depends on $(cc-option,-fpatchable-function-entry=8)

menu "Platform type"
    config NONPORTABLE
        bool "Allow configurations that result in non-portable kernels"
        help
          RISC-V kernel binaries are compatible between all known systems
          whenever possible, but there are some use cases that can only be
          satisfied by configurations that result in kernel binaries that are
          not portable between systems.
endmenu    
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kconfig_file = KconfigFile::new(PathBuf::from("/"), PathBuf::from("Kconfig"));
    let (_remaining, kconfig) =
        parse_kconfig(KconfigInput::new_extra(INPUT, kconfig_file)).unwrap();
    println!("File '{}' contains the following entries:", kconfig.file);
    kconfig.entries.into_iter().for_each(print_entry);
    Ok(())
}

fn print_entry(entry: nom_kconfig::Entry) {
    match entry {
        nom_kconfig::Entry::Config(config) => println!(" - Config '{}'", config.symbol),
        nom_kconfig::Entry::Menu(menu) => {
            menu.entries.into_iter().for_each(print_entry);
        }
        _ => (),
    }
}

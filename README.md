<div align="center">
  <br>
  <img
    alt="Tux, the pinguin"
    src="./doc/tux.png"
    width=100px
  />
  <h1>A Kconfig parser written in rust.</h1>
</div>

[![Build status](https://github.com/Mcdostone/nom-kconfig/actions/workflows/build.yml/badge.svg)](https://github.com/Mcdostone/nom-kconfig/actions/workflows/build.yml)
[![Code coverage](https://codecov.io/gh/Mcdostone/nom-kconfig/branch/main/graph/badge.svg?token=QF0CRBCO2C)](https://codecov.io/gh/Mcdostone/nom-kconfig)
[![Minimum supported rust version: 1.56.0 or plus](https://img.shields.io/badge/MSRV-1.56.0+-lightgray.svg?logo=rust)](https://github.com/rust-bakery/nom#rust-version-requirements-msrv)
[![crates.io Version](https://img.shields.io/crates/v/nom-kconfig.svg?logo=crate)](https://crates.io/crates/nom-kconfig)



Kconfig is a language that describes configuration options for the linux Kernel. The syntax looks like this:
```bash
# https://github.com/torvalds/linux/blob/master/arch/riscv/Kconfig#L771
config EFI
	bool "UEFI runtime support"
	depends on MMU
	default y
	select EFI_STUB
	help
	  This option provides support for runtime services provided
	  by UEFI firmware.
```

- The file starts with a `config` entry: We define a config named `EFI`. The next lines are the attributes of this entry.
- `EFI` is a boolean config.
- `EFI` [depends on](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes) the config `MMU`.
- Its default value is `y`.
- If `EFI` is equals to `true` then it enables `EFI_STUB`.
- The `help` attribute defines a help text for the end user.

There are plenty of other keywords in the Kconfig language, check out [the official documentation](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html) for more details.

**Features**

 - This is a parser, there is no semantic analysis in this library.
 - This library only supports UTF-8 encoded files.
 - List of supported entris can be found [here](https://docs.rs/nom-kconfig/latest/nom_kconfig/entry/enum.Entry.html).
 - List of supported attributes can be found [here](https://docs.rs/nom-kconfig/latest/nom_kconfig/attribute/enum.Attribute.html).
 - When [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-entries) is met, it reads and parses the specified configuration file.
 - This library uses `clone()` a lot. Do not expect amazing performances.
 - This parser has been tested on the linux kernel repository from [2.6.11](https://cdn.kernel.org/pub/linux/kernel/v2.6/linux-2.6.11.tar.xz) to [6.4.9](https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.4.9.tar.xz) (3733 versions).
 

## Getting started

```bash
cargo add nom-kconfig
```

```rust
use std::path::PathBuf;
use nom_kconfig::{kconfig::parse_kconfig, KconfigInput, KconfigFile};

// curl https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.4.9.tar.xz | tar -xJ -C /tmp/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kconfig_file = KconfigFile::new(
        PathBuf::from("/tmp/linux-6.4.9"), 
        PathBuf::from("/tmp/linux-6.4.9/Kconfig")
    );
    let input = kconfig_file.read_to_string().unwrap();
    let kconfig = parse_kconfig(KconfigInput::new_extra(&input, kconfig_file));
    println!("{:?}", kconfig);
    Ok(())
}
```

## Resources
 - https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html
 - https://doc.coreboot.org/getting_started/kconfig.html
 - https://build2.org/libbuild2-kconfig/doc/build2-kconfig-manual.xhtml#lang
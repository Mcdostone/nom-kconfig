<div align="center">
  <br>
  <img
    alt="Tux, the pinguin"
    src="./doc/tux.png"
    width=100px
  />
  <br/>
  <h1>A Kconfig parser written in rust.</h1>
</div>
<br/>
<p align="center">
  <a href="https://github.com/Mcdostone/nom-kconfig/actions/workflows/build.yml">
    <img src="https://github.com/Mcdostone/nom-kconfig/actions/workflows/build.yml/badge.svg" alt="Build status"/>
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.71.0-green.svg?logo=rust" alt="Rust version"/>
  </a>
<a href="https://codecov.io/gh/Mcdostone/nom-kconfig" > 
 <img src="https://codecov.io/gh/Mcdostone/nom-kconfig/branch/main/graph/badge.svg?token=QF0CRBCO2C" alt="code coverage"/> 
 </a>

</p>

Parsing relies on the [nom library](https://github.com/rust-bakery/nom).
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
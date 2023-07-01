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
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.70.0-green.svg?logo=rust" alt="Rust version"/>
  </a>
</p>

Parsing relies on the [nom library](https://github.com/rust-bakery/nom).
## Getting started

```bash
cargo add kconfig-nom
```

```rust
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
extern crate kconfig;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match parser.parse(current_kconfig, input.as_ref()) {
    Ok(())
}
```

## Resources
 - https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html
 - https://doc.coreboot.org/getting_started/kconfig.html
 - https://build2.org/libbuild2-kconfig/doc/build2-kconfig-manual.xhtml#lang
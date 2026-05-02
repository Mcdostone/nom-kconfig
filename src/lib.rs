#![allow(clippy::result_large_err)]

//! # nom-kconfig
//!
//! A parser for kconfig files. The parsing is done with [nom](https://github.com/rust-bakery/nom).
//!
//! ```no_run
//! use std::path::PathBuf;
//! use nom_kconfig::{parse_kconfig, KconfigInput, KconfigFile};
//! use std::collections::HashMap;
//!
//! // curl https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.4.9.tar.xz | tar -xJ -C /tmp/
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut variables = HashMap::new();
//!     variables.insert("SRCARCH", "x86");
//!     let kconfig_file = KconfigFile::new_with_vars(
//!         PathBuf::from("/tmp/linux-6.4.9"),
//!         PathBuf::from("/tmp/linux-6.4.9/Kconfig"),
//!         &variables
//!     );
//!     let input = kconfig_file.read_to_string().unwrap();
//!     let kconfig = parse_kconfig(KconfigInput::new_extra(&input, kconfig_file));
//!     println!("{:?}", kconfig);
//!     Ok(())
//! }
//! ```

pub mod attribute;
pub mod entry;
pub mod kconfig;
pub mod kconfig_file;
pub mod string;
pub mod symbol;
pub mod tristate;
pub mod util;

pub use self::attribute::Attribute;
pub use self::entry::Entry;
pub use self::kconfig::{parse_kconfig, Kconfig};
pub use self::symbol::Symbol;
pub use kconfig_file::KconfigFile;
use nom_locate::LocatedSpan;

/// [KconfigInput] is a struct gathering a [KconfigFile] and its associated content.
pub type KconfigInput<'a> = LocatedSpan<&'a str, KconfigFile>;

#[cfg(test)]
pub mod kconfig_test;
#[cfg(test)]
pub mod lib_test;
mod number;
#[cfg(test)]
pub mod symbol_test;
#[cfg(test)]
pub mod util_test;

#[macro_export]
macro_rules! assert_parsing_eq {
    ($fn:ident, $input:expr, $expected:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra($input, Default::default()))
            .map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}

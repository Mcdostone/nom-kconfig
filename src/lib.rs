#![allow(clippy::result_large_err)]

//! # nom-kconfig
//!
//! A parser for kconfig files. The parsing is done with [nom](https://github.com/rust-bakery/nom).
//!
//! ```no_run
//! use std::path::PathBuf;
//! use nom_kconfig::{parse_kconfig, KconfigInput, KconfigFile};
//!
//! // curl https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.4.9.tar.xz | tar -xJ -C /tmp/
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let kconfig_file = KconfigFile::new(
//!         PathBuf::from("/tmp/linux-6.4.9"),
//!         PathBuf::from("/tmp/linux-6.4.9/Kconfig")
//!     );
//!     let input = kconfig_file.read_to_string().unwrap();
//!     let kconfig = parse_kconfig(KconfigInput::new_extra(&input, kconfig_file));
//!     println!("{:?}", kconfig);
//!     Ok(())
//! }
//! ```

use nom_locate::LocatedSpan;

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

pub mod attribute;
pub mod entry;
pub mod kconfig;
pub mod symbol;
pub mod util;

pub use self::attribute::Attribute;
pub use self::entry::Entry;
pub use self::kconfig::{parse_kconfig, Kconfig};
pub use self::symbol::Symbol;

/// [KconfigInput] is a struct gathering a [KconfigFile] and its associated content.
pub type KconfigInput<'a> = LocatedSpan<&'a str, KconfigFile>;

/// Represents a Kconfig file.
/// It stores the kernel root directory because we need this information when a [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#kconfig-syntax) keyword is met.
#[derive(Debug, Default, Clone)]
pub struct KconfigFile {
    /// The absolute path of the kernel root directory. This field is necessary to parse [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#kconfig-syntax) entry.
    root_dir: PathBuf,
    /// The path the the Kconfig you want to parse.
    file: PathBuf,
    /// Externally-specified variables to use when including child source files
    vars: HashMap<String, String>,
}

impl KconfigFile {
    pub fn new(root_dir: PathBuf, file: PathBuf) -> Self {
        Self {
            root_dir,
            file,
            vars: HashMap::new(),
        }
    }

    pub fn new_with_vars<S: AsRef<str>>(
        root_dir: PathBuf,
        file: PathBuf,
        vars: &HashMap<S, S>,
    ) -> Self {
        Self {
            root_dir,
            file,
            vars: vars
                .iter()
                .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                .collect(),
        }
    }

    pub fn full_path(&self) -> PathBuf {
        self.root_dir.join(&self.file)
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self.full_path())
    }

    pub fn set_vars<S: AsRef<str>>(&mut self, vars: &[(S, S)]) {
        self.vars = vars
            .iter()
            .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
            .collect();
    }
}

#[cfg(test)]
pub mod kconfig_test;
#[cfg(test)]
pub mod lib_test;
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

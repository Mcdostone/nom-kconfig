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

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

/// Represents a Kconfig file.
/// It stores the kernel root directory because we need this information when a [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#kconfig-syntax) keyword is met.
#[derive(Debug, Default, Clone)]
pub struct KconfigFile {
    /// The absolute path of the kernel root directory. This field is necessary to parse [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#kconfig-syntax) entry.
    pub root_dir: PathBuf,
    /// The path the the Kconfig you want to parse.
    pub file: PathBuf,
    /// Externally-specified variables to use when including child source files
    pub global_vars: HashMap<String, String>,
    pub local_vars: HashMap<String, String>,
}

impl KconfigFile {
    pub fn new(root_dir: PathBuf, file: PathBuf) -> Self {
        Self {
            root_dir,
            file,
            global_vars: HashMap::new(),
            local_vars: HashMap::new(),
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
            global_vars: vars
                .iter()
                .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                .collect(),
            local_vars: HashMap::new(),
        }
    }

    pub fn vars(&self) -> HashMap<String, String> {
        self.global_vars
            .iter()
            .chain(self.local_vars.iter())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn full_path(&self) -> PathBuf {
        match self.file.is_absolute() {
            true => self.file.clone(),
            false => self.root_dir.join(&self.file),
        }
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self.full_path())
    }

    pub fn set_vars<S: AsRef<str>>(&mut self, vars: &[(S, S)]) {
        self.global_vars = vars
            .iter()
            .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
            .collect();
    }

    pub fn add_local_var<S: AsRef<str>>(&mut self, key: S, value: S) {
        self.local_vars
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
    }
}

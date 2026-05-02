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

use regex::Regex;

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
    pub depth: usize,
}

impl KconfigFile {
    pub fn new(root_dir: PathBuf, file: PathBuf) -> Self {
        Self {
            root_dir,
            file,
            global_vars: HashMap::new(),
            local_vars: HashMap::new(),
            depth: 0,
        }
    }

    pub fn new_with_vars<S: AsRef<str>>(
        root_dir: PathBuf,
        file: PathBuf,
        global_vars: &HashMap<S, S>,
        local_vars: &HashMap<S, S>,
    ) -> Self {
        Self {
            root_dir,
            file,
            global_vars: global_vars
                .iter()
                .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                .collect(),
            local_vars: local_vars
                .iter()
                .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                .collect(),
            depth: 0,
        }
    }

    pub fn new_source_file(&self, path: PathBuf) -> Self {
        let mut copied = self.clone();
        copied.file = path;
        copied.depth += 1;
        copied
    }

    pub fn vars(&self) -> &HashMap<String, String> {
        &self.local_vars
    }

    pub fn global_vars(&self) -> &HashMap<String, String> {
        &self.global_vars
    }

    pub fn full_path(&self) -> PathBuf {
        match self.file.is_absolute() {
            true => self.file.clone(),
            false => self.root_dir.join(&self.file),
        }
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self.full_path())
            .map(|content| self.preprocess_variables(content))
            .map(|content| self.preprocess_macros(content))
    }

    pub fn set_global_vars<S: AsRef<str>>(&mut self, vars: &[(S, S)]) {
        self.global_vars = vars
            .iter()
            .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
            .collect();
    }

    pub fn add_local_var<S: AsRef<str>>(&mut self, key: S, value: S) {
        self.local_vars
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
    }

    pub fn add_local_vars(&mut self, new_vars: HashMap<String, String>) {
        self.local_vars.extend(new_vars);
    }

    fn preprocess_variables(&self, content: String) -> String {
        let regex: Regex = Regex::new(r"\$\(([a-zA-Z_][a-zA-Z0-9_-]*)\)").unwrap();
        let mut processed_content = content.clone();
        let variables = self.vars();
        for (var_name, var_value) in regex.captures_iter(&content).map(|cap| {
            let ex: (&str, [&str; 1]) = cap.extract();
            let var = ex.1[0];
            (var, variables.get(var))
        }) {
            if let Some(var_value) = var_value {
                processed_content = processed_content.replace(&format!("$({var_name})"), var_value);
            }
        }
        processed_content
    }

    pub fn preprocess_macros(&self, content: String) -> String {
        let re = Regex::new(r"\$\((\S+)\)").unwrap();
        let variables = self.vars();
        let mut file_copy = content.clone();
        for (var_name, var_value) in re.captures_iter(&content).map(|cap| {
            let ex: (&str, [&str; 1]) = cap.extract();
            let var = ex.1[0];
            (var, variables.get(var))
        }) {
            if let Some(var_value) = var_value {
                file_copy = file_copy.replace(&format!("$({var_name})"), var_value);
            }
        }

        let re = Regex::new(r"\$\{(\S+)\}").unwrap();
        let mut file_copy_2 = file_copy.clone();
        for (var_name, var_value) in re.captures_iter(&file_copy).map(|cap| {
            let ex: (&str, [&str; 1]) = cap.extract();
            let var = ex.1[0];
            (var, variables.get(var))
        }) {
            if let Some(var_value) = var_value {
                file_copy_2 = file_copy_2.replace(&format!("${{{var_name}}}"), var_value);
            }
        }

        file_copy_2
    }
}

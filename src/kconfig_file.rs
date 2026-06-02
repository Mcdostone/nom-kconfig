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
//!         &variables,
//!         &HashMap::default(),
//!     );
//!     let input = kconfig_file.read_to_string().unwrap();
//!     let kconfig = parse_kconfig(KconfigInput::new_extra(&input, kconfig_file));
//!     println!("{:?}", kconfig);
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::{fs, io};

/// Represents a Kconfig file.
/// It stores the kernel root directory because we need this information when a [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#kconfig-syntax) keyword is met.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KconfigFile {
    /// The absolute path of the kernel root directory. This field is necessary to parse [`source`](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#kconfig-syntax) entry.
    pub root_dir: PathBuf,
    /// The path the the Kconfig you want to parse.
    pub file: PathBuf,
    /// Externally-specified variables to use when including child source files
    pub global_vars: Rc<HashMap<String, String>>,
    pub local_vars: Rc<HashMap<String, String>>,
    pub external_functions: Rc<HashMap<String, String>>,
    pub depth: usize,
    pub parent_file: Option<PathBuf>,
}

impl KconfigFile {
    pub fn new(root_dir: PathBuf, file: PathBuf) -> Self {
        Self {
            root_dir,
            file,
            global_vars: Rc::new(HashMap::new()),
            local_vars: Rc::new(HashMap::new()),
            external_functions: Rc::new(HashMap::new()),
            depth: 0,
            parent_file: None,
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
            global_vars: Rc::new(
                global_vars
                    .iter()
                    .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                    .collect(),
            ),
            local_vars: Rc::new(
                local_vars
                    .iter()
                    .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                    .collect(),
            ),
            external_functions: Rc::new(HashMap::new()),
            depth: 0,
            parent_file: None,
        }
    }

    pub fn with_external_functions(mut self, external_functions: &HashMap<String, String>) -> Self {
        self.external_functions = Rc::new(external_functions.clone());
        self
    }

    pub fn new_source_file(&self, path: PathBuf) -> Self {
        let mut copied = self.clone();
        copied.file = path;
        copied.depth += 1;
        copied.parent_file = Some(self.file.clone());
        copied
    }

    pub fn vars(&self) -> HashMap<String, String> {
        let mut variables = (*self.global_vars).clone();
        variables.extend((*self.local_vars).clone());
        variables
    }

    pub fn global_vars(&self) -> &HashMap<String, String> {
        &self.global_vars
    }

    pub fn full_path(&self) -> PathBuf {
        self.root_dir.join(&self.file)
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self.full_path()).map(|content| self.preprocess_content(content))
    }

    pub fn set_global_vars<S: AsRef<str>>(&mut self, vars: &[(S, S)]) {
        self.global_vars = Rc::new(
            vars.iter()
                .map(|(s1, s2)| (s1.as_ref().to_string(), s2.as_ref().to_string()))
                .collect(),
        );
    }

    pub fn add_local_var<S: AsRef<str>>(&mut self, key: S, value: S) {
        let mut new_map = (*self.local_vars).clone();
        new_map.insert(key.as_ref().to_string(), value.as_ref().to_string());
        self.local_vars = Rc::new(new_map);
    }

    pub fn add_local_vars(&mut self, new_vars: HashMap<String, String>) {
        if new_vars.is_empty() {
            return;
        }
        let mut new_map = (*self.local_vars).clone();
        new_map.extend(new_vars);
        self.local_vars = Rc::new(new_map);
    }

    pub fn preprocess_content(&self, content: String) -> String {
        let variables = self.vars();
        if variables.is_empty() {
            return content;
        }
        let mut file_copy = content.clone();
        for (var_name, var_value) in variables {
            file_copy = file_copy.replace(&format!("$({var_name})"), &var_value);
            file_copy = file_copy.replace(&format!("${{{var_name}}}"), &var_value);
        }

        file_copy
    }
}

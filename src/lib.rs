use nom_locate::LocatedSpan;

use std::path::PathBuf;
use std::{fs, io};

pub mod attribute;
pub mod entry;
pub mod kconfig;
pub mod symbol;
pub mod util;

pub use self::attribute::Attribute;
pub use self::entry::Entry;
pub use self::kconfig::Kconfig;

pub type KconfigInput<'a> = LocatedSpan<&'a str, KconfigFile>;

#[derive(Debug, Clone, Default)]
pub struct KconfigFile {
    pub root_dir: PathBuf,
    pub file: PathBuf,
    pub fail_on_missing_source: bool,
}

impl KconfigFile {
    pub fn new(root_dir: PathBuf, file: PathBuf, fail_on_missing_source: bool) -> Self {
        Self {
            root_dir,
            file,
            fail_on_missing_source,
        }
    }

    pub fn full_path(&self) -> PathBuf {
        self.root_dir.join(&self.file)
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        let input = fs::read_to_string(self.full_path())?;
        /*let re = Regex::new("\\\\\n").unwrap();
        let input = re.replace_all(&input, "");
        let re = Regex::new("\t").unwrap();
        let input = re.replace_all(&input, "    ");*/
        Ok(input.to_string())
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

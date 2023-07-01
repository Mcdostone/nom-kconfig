use nom::{combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::{
    entry::{parse_entry, Entry},
    util::ws_comment,
    KconfigInput,
};

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Kconfig {
    pub file: String,
    pub entries: Vec<Entry>,
}

pub fn parse_kconfig(input: KconfigInput) -> IResult<KconfigInput, Kconfig> {
    let file = input.extra.file.clone();
    let ok = map(delimited(ws_comment, many0(parse_entry), ws_comment), |d| {
        Kconfig {
            file: file.display().to_string(),
            entries: d,
        }
    })(input);
    // TODO
    #[allow(clippy::let_and_return)]
    ok
}

/*fn is_architecture_related(kconfig: &PathBuf, kernel_dir: &PathBuf) -> Option<String> {
    match kconfig.starts_with(kernel_dir.join("arch")) {
        true => {
            let ok = kconfig
                .strip_prefix(kernel_dir)
                .unwrap()
                .components()
                .nth(1)
                .unwrap();
            Some(ok.to_owned().as_os_str().to_str().unwrap().to_string())
        }
        false => None,
    }
}*/

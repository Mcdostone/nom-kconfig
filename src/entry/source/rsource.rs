use crate::{entry::Source, util::ws, KconfigInput};
use nom::{bytes::complete::tag, IResult, Parser};

pub type RSource = Source;

#[allow(dead_code)]
pub fn parse_rsource(input: KconfigInput) -> IResult<KconfigInput, RSource> {
    let (_input, _) = ws(tag("rsource")).parse(input)?;
    todo!("need to implemnt rsource");
    //Ok((input, RSource(kconfigs: vec![Kconfig
    //        { kconfigs: vec![] })))
}

#[cfg(test)]
use crate::KconfigFile;
#[cfg(test)]
use std::path::PathBuf;

#[test]
#[ignore]
fn test_parse_rsource() {
    let res = parse_rsource(KconfigInput::new_extra(
        "rsource glob-fixtures/does-not-exist-*.Kconfig",
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

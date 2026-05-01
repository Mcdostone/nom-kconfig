#[cfg(feature = "kconfiglib")]
mod orsource;
#[cfg(feature = "kconfiglib")]
mod osource;
#[cfg(feature = "kconfiglib")]
mod rsource;
#[allow(clippy::module_inception)]
mod source;

use nom::{
    branch::alt,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    IResult, Parser,
};

#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
#[cfg(feature = "kconfiglib")]
pub use self::{
    orsource::parse_orsource, orsource::OrSource, osource::parse_osource, osource::OSource,
    rsource::parse_rsource, rsource::RSource,
};
pub use source::{parse_source, Source};
use tracing::{debug, error};

use crate::KconfigInput;
use crate::{parse_kconfig, util::ws, Kconfig, KconfigFile};

#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
pub use glob::glob;
#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
use std::path::PathBuf;

#[cfg(test)]
mod source_test;

#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
enum JoinPathMode {
    Relative,

    Root,
}

pub(crate) fn parse_filepath(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, &str> {
    map(
        recognize(ws(many1(alt((
            alphanumeric1::<KconfigInput, _>,
            recognize(one_of(".$(){}*-_$+@/")),
        ))))),
        |d| d.fragment().to_owned(),
    )
    .parse(input)
}

fn parse_source_kconfig(
    input: KconfigInput,
    source_kconfig_file: KconfigFile,
) -> Result<Kconfig, nom::Err<Error<KconfigInput>>> {
    let source_content = source_kconfig_file
        .read_to_string()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)));

    #[cfg(feature = "kconfiglib")]
    {
        // TODO
        // if the file doesn't exist, it's probably because the filename is dynamically generated with macros/variables.
        // In that case, we can return an empty Kconfig instead of failing to parse the source file.
        //
        // This is not the best solution !
        if source_content.is_err() {
            error!(
                "I tried to parse the source file '{}' defined in '{}'. This is likely because the filename is dynamically generated with macros/variables that are not supported yet. Returning an empty Kconfig for this source file.",
                source_kconfig_file.full_path().display(),
                input.extra.full_path().display()
            );
            return Ok(Kconfig {
                file: source_kconfig_file.full_path().display().to_string(),
                entries: vec![],
            });
        }
    }

    let source_content = source_content?;

    #[allow(clippy::let_and_return)]
    let x = match cut(parse_kconfig).parse(KconfigInput::new_extra(
        &source_content,
        source_kconfig_file.clone(),
    )) {
        Ok((_, kconfig)) => Ok(kconfig),
        Err(e) => {
            debug!("Variables are {:?}", input.extra.vars());
            error!(
                "Failed to parse source file '{}'",
                source_kconfig_file.full_path().display(),
            );
            error!(
                "The source file is defined in '{}'",
                input.extra.full_path().display()
            );
            match e {
                nom::Err::Incomplete(needed) => error!("Incomplete parsing: {:?}", needed),
                nom::Err::Error(e) => error!(
                    "error is due to '{:?}' parsing the content '{:?}'",
                    e.code,
                    e.input.fragment()[..std::cmp::min(100, e.input.fragment().len())].to_string()
                ),
                nom::Err::Failure(e) => error!(
                    "error is due to '{:?}' parsing the content '{:?}'",
                    e.code,
                    e.input.fragment()[..std::cmp::min(100, e.input.fragment().len())].to_string()
                ),
            }
            Err(nom::Err::Error(Error::new(
                KconfigInput::new_extra("", source_kconfig_file),
                ErrorKind::Fail,
            )))
        }
    };
    x
}

#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
fn expand_source_files<'a>(
    input: KconfigInput<'a>,
    file: &str,
    mode: JoinPathMode,
) -> Result<Vec<PathBuf>, nom::Err<Error<KconfigInput<'a>>>> {
    let mut expanded_files = Vec::new();
    let prefix_path = match mode {
        JoinPathMode::Relative => input.extra.full_path().parent().unwrap().to_path_buf(),
        JoinPathMode::Root => input.extra.root_dir.clone(),
    };

    let full_path_pattern = prefix_path.join(file);
    let paths: Vec<PathBuf> = glob(&full_path_pattern.display().to_string())
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;

    if paths.is_empty() {
        return Ok(vec![prefix_path.join(file)]);
    }
    for source_path in paths {
        let source_path = source_path.canonicalize().unwrap();
        let source_path_without_root = source_path
            // TODO need to canonicalize because of macOS weird filepath for temp directories
            // /var/folder
            // and /private/var/folder
            .strip_prefix(input.extra.root_dir.canonicalize().unwrap())
            .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;
        expanded_files.push(source_path_without_root.to_path_buf());
    }

    expanded_files.sort();
    if expanded_files.is_empty() {
        debug!("No expanded files found");
        return Err(nom::Err::Error(Error::from_error_kind(
            input,
            ErrorKind::Fail,
        )));
    }

    Ok(expanded_files)
}

#[cfg(test)]
use crate::assert_parsing_eq;

#[test]
fn test_parse_filepath() {
    assert_parsing_eq!(
        parse_filepath,
        "u-boot/board/sagem/f@st1704/Kconfig",
        Ok(("", "u-boot/board/sagem/f@st1704/Kconfig"))
    );

    assert_parsing_eq!(
        parse_filepath,
        "u-boot/board/l+g/vinco/Kconfig",
        Ok(("", "u-boot/board/l+g/vinco/Kconfig"))
    );
}

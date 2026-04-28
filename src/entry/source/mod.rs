#[cfg(feature = "kconfiglib")]
mod orsource;
#[cfg(feature = "kconfiglib")]
mod osource;
#[cfg(feature = "kconfiglib")]
mod rsource;
#[allow(clippy::module_inception)]
mod source;

use nom::lib::std::collections::HashMap;
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
use regex::Regex;
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
    debug!(
        "Parsing source file '{}' from '{}'",
        source_kconfig_file.full_path().display(),
        input.extra.full_path().display()
    );

    let source_content = source_kconfig_file
        .read_to_string()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;

    #[allow(clippy::let_and_return)]
    let x = match cut(parse_kconfig).parse(KconfigInput::new_extra(
        &source_content,
        source_kconfig_file.clone(),
    )) {
        Ok((_, kconfig)) => Ok(kconfig),
        Err(e) => {
            debug!("Variables are {:?}", input.extra.vars());
            error!("Failed to parse source file '{:?}'", e);
            Err(nom::Err::Error(Error::new(
                KconfigInput::new_extra("", source_kconfig_file),
                ErrorKind::Fail,
            )))
        }
    };
    x
}

pub fn apply_vars(content: &str, extra_vars: &HashMap<String, String>) -> Option<String> {
    apply_vars_for_pattern(
        content,
        extra_vars,
        Regex::new(r"\$\(([a-zA-Z_][a-zA-Z0-9_-]*)\)").unwrap(),
        true,
    ) // {
      //   Some(result) => Some(result),
      //   None => match apply_vars_for_pattern(
      //       content,
      //       extra_vars,
      //       Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_-]*)\}").unwrap(),
      //       false
      //   ) {
      //       Some(result) => Some(result),
      //       None => None,
      //   }
      //}//
}

fn apply_vars_for_pattern(
    content: &str,
    extra_vars: &HashMap<String, String>,
    regex: Regex,
    parenthesis: bool,
) -> Option<String> {
    let mut file_copy = String::from(content);
    for (var_name, var_value) in regex.captures_iter(content).map(|cap| {
        let ex: (&str, [&str; 1]) = cap.extract();
        let var = ex.1[0];
        (var, extra_vars.get(var))
    }) {
        if let Some(var_value) = var_value {
            file_copy = match parenthesis {
                true => file_copy.replace(&format!("$({var_name})"), var_value),
                false => file_copy.replace(&format!("${{{var_name}}}"), var_value),
            }
        } else {
            return None;
        }
    }
    Some(file_copy)
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
            .strip_prefix(&input.extra.root_dir)
            .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;
        expanded_files.push(source_path_without_root.to_path_buf());
    }

    expanded_files.sort();
    if expanded_files.is_empty() {
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

#[test]
#[ignore]
fn test_apply_vars() {
    assert_eq!(
        apply_vars(
            "${ZEPHYR_BASE}/subsys/logging/Kconfig.template.log_config",
            &HashMap::from([("ZEPHYR_BASE".to_string(), "/zephyr".to_string())]),
        ),
        Some("/zephyr/subsys/logging/Kconfig.template.log_config".to_string())
    );

    assert_eq!(
        apply_vars(
            "$(ZEPHYR_BASE/subsys/logging/Kconfig.template.log_config",
            &HashMap::from([("ZEPHYR_BASE".to_string(), "/zephyr".to_string())]),
        ),
        Some("/zephyr/subsys/logging/Kconfig.template.log_config".to_string())
    );
}

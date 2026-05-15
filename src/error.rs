use std::{fmt, path::PathBuf};

use nom::Needed;

use crate::KconfigInput;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub filename: PathBuf,
    pub resolved_path: PathBuf,
    pub parent_file: Option<PathBuf>,
    pub line: u32,
    pub column: usize,
    pub kind: ErrorKind,
    pub input: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Nom(nom::error::ErrorKind),
    Unknown,
    Incomplete,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Nom(kind) => write!(f, "{}", kind.description()),
            ErrorKind::Unknown => write!(f, "unknown"),
            ErrorKind::Incomplete => write!(f, "incomplete"),
        }
    }
}

impl std::error::Error for Error {}

impl From<nom::Err<nom::error::Error<KconfigInput<'_>>>> for Error {
    fn from(value: nom::Err<nom::error::Error<KconfigInput>>) -> Self {
        match value {
            nom::Err::Incomplete(Needed::Size(_u)) => Error {
                filename: Default::default(),
                resolved_path: Default::default(),
                parent_file: None,
                line: 0,
                column: 0,
                input: "Input is not available".to_string(),
                kind: ErrorKind::Incomplete,
            },
            nom::Err::Incomplete(Needed::Unknown) => Error {
                filename: Default::default(),
                resolved_path: Default::default(),
                parent_file: None,
                line: 0,
                column: 0,
                input: "Input is not available".to_string(),
                kind: ErrorKind::Incomplete,
            },
            nom::Err::Error(error) => Error {
                filename: error.input.extra.file.clone(),
                resolved_path: error.input.extra.full_path(),
                parent_file: error.input.extra.parent_file.clone(),
                line: error.input.location_line(),
                column: error.input.location_offset(),
                input: error.input.fragment().to_string(),
                kind: ErrorKind::Nom(error.code),
            },
            nom::Err::Failure(failure) => Error {
                filename: failure.input.extra.file.clone(),
                resolved_path: failure.input.extra.full_path(),
                parent_file: failure.input.extra.parent_file.clone(),
                line: failure.input.location_line(),
                column: failure.input.location_offset(),
                input: failure.input.fragment().to_string(),
                kind: ErrorKind::Nom(failure.code),
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"error[{}]: Something went wrong while parsing at the following location:
    Input file: {}:{}:{}
 Resolved path: {}:{}:{}
   Parent file: {}

The parser failed at the following input:
{}"#,
            self.kind,
            self.filename.display(),
            self.line,
            self.column,
            self.resolved_path.display(),
            self.line,
            self.column,
            self.parent_file
                .as_ref()
                .map(|e| e.display().to_string())
                .unwrap_or_default(),
            self.input.lines().take(10).collect::<Vec<_>>().join("\n"),
        )
    }
}

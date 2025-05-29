use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FormatError {
    ParseError(String),
    FormatError(String),
    UnsupportedFormat(String),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FormatError::FormatError(msg) => write!(f, "Format error: {}", msg),
            FormatError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
        }
    }
}

impl std::error::Error for FormatError {}

pub type FormatResult = Result<String, FormatError>;

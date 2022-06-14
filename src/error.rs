use serde::de;
use serde::ser;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    code: ErrorCode,
}

impl From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        Self { code }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Direction {
    Serialization,
    Deserialization,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Serialization => f.write_str("serialization"),
            Direction::Deserialization => f.write_str("deserialization"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum ErrorCode {
    Message(String),
    UnsupportedOperation(Direction, String),
    InvalidType {
        unexpected: String,
        expected: String,
    },
    InvalidVariantName {
        received: String,
        allowed: Vec<String>,
    },
    TrailingCharacters,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::UnsupportedOperation(d, t) => {
                write!(f, "unsupported operation: {}: {}", d, t)
            }
            ErrorCode::InvalidType {
                unexpected,
                expected,
            } => write!(f, "invalid type: {}, expected {}", unexpected, expected),
            ErrorCode::InvalidVariantName { received, allowed } => {
                write!(
                    f,
                    "invalid variant: {} is not a valid variant name ({:?})",
                    received, allowed
                )
            }
            ErrorCode::TrailingCharacters => write!(
                f,
                "trailing characters: input ends with trailing characters"
            ),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.code, f)
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            code: ErrorCode::Message(format!("{}", msg)),
        }
    }

    #[cold]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Error {
            code: ErrorCode::InvalidVariantName {
                received: variant.to_string(),
                allowed: expected.iter().map(|v| v.to_string()).collect(),
            },
        }
    }

    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error {
            code: ErrorCode::InvalidType {
                unexpected: format!("{}", unexp),
                expected: format!("{}", exp),
            },
        }
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            code: ErrorCode::Message(format!("{}", msg)),
        }
    }
}

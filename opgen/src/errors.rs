use std::fmt::{self, Display};

#[derive(Debug)]
pub enum BuildError {
    Fmt(std::io::Error),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Fmt(error) => write!(f, "failed to run rustfmt: {}", error),
        }
    }
}

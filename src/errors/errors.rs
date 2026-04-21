use std::fmt;

#[derive(Debug)]
pub enum RaytracerError {
    IncorrectArguments,
    IoError(std::io::Error),
}

impl fmt::Display for RaytracerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RaytracerError::IncorrectArguments => write!(f, "Incorrect arguments!"),
            RaytracerError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for RaytracerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RaytracerError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RaytracerError {
    fn from(e: std::io::Error) -> Self {
        RaytracerError::IoError(e)
    }
}
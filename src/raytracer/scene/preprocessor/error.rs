#[derive(Debug)]
pub enum Error {
    Ron(ron::error::SpannedError),
    Ser(ron::Error),
    Io(std::io::Error),
    CyclicDependency(Vec<String>),
    UndefinedRef(String),
    MissingProp(String, String),
    InvalidStructure(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Ron(e) => write!(f, "RON parse error: {e}"),
            Error::Ser(e) => write!(f, "RON serialize error: {e}"),
            Error::Io(e) => write!(f, "IO error: {e}"),
            Error::CyclicDependency(s) => write!(f, "Cyclic dependency: {}", s.join(" → ")),
            Error::UndefinedRef(n) => write!(f, "Undefined reference: @{n}"),
            Error::MissingProp(c, p) => write!(f, "Missing prop '{p}' for '{c}'"),
            Error::InvalidStructure(m) => write!(f, "Invalid structure: {m}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ron::error::SpannedError> for Error {
    fn from(e: ron::error::SpannedError) -> Self {
        Error::Ron(e)
    }
}
impl From<ron::Error> for Error {
    fn from(e: ron::Error) -> Self {
        Error::Ser(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

#[derive(Debug)]
pub enum ProleError {
    Exit(String),
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::string::FromUtf8Error),
}

impl std::fmt::Display for ProleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Exit(e) => write!(f, "{}", e),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ParseFloatError(e) => write!(f, "Parse error: {}", e),
            Self::ParseIntError(e) => write!(f, "Parse error: {}", e),
            Self::Utf8Error(e) => write!(f, "UTF8 error: {}", e),
        }
    }
}

impl std::error::Error for ProleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Exit(_) => None,
            Self::IoError(e) => Some(e),
            Self::ParseFloatError(e) => Some(e),
            Self::ParseIntError(e) => Some(e),
            Self::Utf8Error(e) => Some(e),
        }
    }
}


pub type ProleResult<T> = Result<T, ProleError>;

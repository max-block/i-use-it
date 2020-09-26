#[derive(Debug)]
pub enum AppError {
    PyPISummaryError,
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IOError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::ReqwestError(e)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        AppError::TomlDeError(e)
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(e: toml::ser::Error) -> Self {
        AppError::TomlSerError(e)
    }
}

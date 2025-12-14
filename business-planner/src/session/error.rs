use std::{fmt, io::Error as IoError};
use ciborium::{de::Error as CiboriumDeserError, ser::Error as CiboriumSerError, value::Error as CiboriumValueError};

pub enum SaveSessionError {
    UndefinedSavePath,
    FileExists,
    WriteFileError(IoError),
    CiboriumAsBytesError,
    CiboriumSerializationError(CiboriumValueError),
    CiboriumWriteError(CiboriumSerError<IoError>)
}

impl fmt::Debug for SaveSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match self {
            Self::UndefinedSavePath => "File was not previously saved at any location.".to_string(),
            Self::FileExists => "File exists and overwrite flag is not set. Refusing to overwrite".to_string(),
            Self::WriteFileError(error) => format!("Failed to write the session to file due to {:#?}", error),
            Self::CiboriumAsBytesError => "Could not convert Ciborium value to bytes".to_string(),
            Self::CiboriumSerializationError(error) => format!("{error}"),
            Self::CiboriumWriteError(error) => format!("{error}"),
        };
        write!(f, "{}", error)
    }
}

impl From<IoError> for SaveSessionError {
    fn from(value: IoError) -> Self {
        SaveSessionError::WriteFileError(value)
    }
}

impl From<CiboriumValueError> for SaveSessionError {
    fn from(value: CiboriumValueError) -> Self {
        SaveSessionError::CiboriumSerializationError(value)
    }
}

impl From<CiboriumSerError<IoError>> for SaveSessionError {
    fn from(value: CiboriumSerError<IoError>) -> Self {
        SaveSessionError::CiboriumWriteError(value)
    }
}

#[derive(Debug)]
pub enum LoadSessionError {
    ReadFileError(IoError),
    CiboriumDeserializationError(CiboriumDeserError<IoError>),
}

impl fmt::Display for LoadSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match self {
            Self::ReadFileError(error) => format!("Failed to read file due to error {:#?}", error),
            Self::CiboriumDeserializationError(error) => format!("{error}"),
        };
        write!(f, "{}", error)
    }
}

impl From<IoError> for LoadSessionError {
    fn from(value: IoError) -> Self {
        LoadSessionError::ReadFileError(value)
    }
}

impl From<CiboriumDeserError<IoError>> for LoadSessionError {
    fn from(value: CiboriumDeserError<IoError>) -> Self {
        LoadSessionError::CiboriumDeserializationError(value)
    }
}
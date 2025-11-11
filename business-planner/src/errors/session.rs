use std::{fmt, io::Error as IoError};
use serde_xml_rs::Error as SerdeXmlError;

pub enum SaveSessionError {
    UndefinedSavePath,
    WriteFileError(IoError),
    XmlSerializationError(SerdeXmlError),
}

impl fmt::Debug for SaveSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match self {
            Self::UndefinedSavePath => { "File was not previously saved at any location. Use `save <PATH>` to define the location to save to.".to_string() },
            Self::WriteFileError(error) => { format!("Failed to write the session to file due to {:#?}", error) },
            Self::XmlSerializationError(error) => { format!("Failed to serialize the session to write to file due to {:#?}", error) },
        };
        write!(f, "{}", error)
    }
}

impl From<IoError> for SaveSessionError {
    fn from(value: IoError) -> Self {
        SaveSessionError::WriteFileError(value)
    }
}

impl From<SerdeXmlError> for SaveSessionError {
    fn from(value: SerdeXmlError) -> Self {
        SaveSessionError::XmlSerializationError(value)
    }
}

#[derive(Debug)]
pub enum LoadSessionError {
    ReadFileError(IoError),
    XmlDeserializationError(SerdeXmlError),
}

impl fmt::Display for LoadSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match self {
            Self::ReadFileError(error) => format!("Failed to read file due to error {:#?}", error),
            Self::XmlDeserializationError(error) => format!("Failed to recover saved session from file content due to error {:#?}", error),
        };
        write!(f, "{}", error)
    }
}

impl From<IoError> for LoadSessionError {
    fn from(value: IoError) -> Self {
        LoadSessionError::ReadFileError(value)
    }
}

impl From<SerdeXmlError> for LoadSessionError {
    fn from(value: SerdeXmlError) -> Self {
        LoadSessionError::XmlDeserializationError(value)
    }
}
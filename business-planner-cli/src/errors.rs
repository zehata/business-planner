
use business_planner::session::error::SaveSessionError;
use inquire::InquireError;
use std::{fmt, io::Error as IoError};
use clap::error::Error as ClapError;

pub enum Error {
    BusinessPlannerError(SaveSessionError),
    IoError(IoError),
    ParseError(ParseError),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Error::BusinessPlannerError(error) => format!("{:#?}", error),
            Error::IoError(error) => format!("IO Error: {:#?}", error),
            Error::ParseError(error) => format!("{:#?}", error),
        })
    }
}

pub enum ParseError {
    ClapError(ClapError),
    InquireError(InquireError),
    InvalidCommandError(String),
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ParseError::ClapError(error) => format!("{}", error),
            ParseError::InquireError(error) => format!("{:#?}", error),
            ParseError::InvalidCommandError(command) => format!("{} was not found in the list of valid commands", command),
        })
    }
}

impl From<SaveSessionError> for Error {
    fn from(value: SaveSessionError) -> Self {
        Error::BusinessPlannerError(value)
    }
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Error::IoError(value)
    }
}

impl From<InquireError> for Error {
    fn from(value: InquireError) -> Self {
        Error::ParseError(ParseError::InquireError(value))
    }
}

impl From<ClapError> for Error {
    fn from(value: ClapError) -> Self {
        Error::ParseError(ParseError::ClapError(value))
    }
}
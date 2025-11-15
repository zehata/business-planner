
use business_planner::api::{plugins::PluginDiscoveryError, reports::ReportGenerationError, session::SaveSessionError};
use inquire::InquireError;
use std::{fmt, io::Error as IoError};
use clap::error::Error as ClapError;

#[derive(Debug)]
pub enum Error {
    ReportGenerationError(ReportGenerationError),
    BusinessPlannerError(SaveSessionError),
    IoError(IoError),
    ParseError(ParseError),
    PluginDiscoveryError(PluginDiscoveryError),
}

impl From<ReportGenerationError> for Error {
    fn from(value: ReportGenerationError) -> Self {
        Error::ReportGenerationError(value)
    }
}

impl From<PluginDiscoveryError> for Error {
    fn from(value: PluginDiscoveryError) -> Self {
        Error::PluginDiscoveryError(value)
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

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::ParseError(value)
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
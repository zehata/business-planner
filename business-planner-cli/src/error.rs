use std::{fmt, io::Error as IoError};

use business_planner::api::error::BusinessPlannerError;
pub use clap::Error as ClapError;
pub use inquire::InquireError;
pub use strum::ParseError;
pub use uuid::Error as UuidError;
pub use dialoguer::Error as DialoguerError;

pub enum NonError {
    Continue,
    Exit,
}

#[derive(Debug)]
pub enum Error {
    DialoguerError(DialoguerError),
    IoError(IoError),
    InvalidInput,
    NotFound(String),
    MultipleFound(String),
    ParseError(ParseError),
    UserCancelled,
    BusinessPlannerError(BusinessPlannerError),
    ClapError(ClapError),
    InquireError(InquireError),
    UuidError(UuidError),
    ErroneousShlexInput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DialoguerError(error) => write!(f, "{}", error),
            Error::IoError(error) => write!(f, "{}", error),
            Error::ParseError(_) => write!(f, "Input is invalid"),
            Error::InvalidInput => write!(f, "Input is invalid"),
            Error::NotFound(item) => write!(f, "No {} found", item),
            Error::MultipleFound(item) => write!(f, "Multiple {} found", item),
            Error::UserCancelled => write!(f, "Cancelled"),
            Error::BusinessPlannerError(business_planner_error) => {
                write!(f, "{}", business_planner_error)
            }
            Error::ClapError(clap_error) => write!(f, "{clap_error}"),
            Error::InquireError(inquire_error) => write!(f, "{inquire_error}"),
            Error::UuidError(uuid_error) => write!(f, "{uuid_error}"),
            Error::ErroneousShlexInput => write!(f, "Erroneous input provided to command splitter"),
        }
    }
}

impl From<DialoguerError> for Error {
    fn from(value: DialoguerError) -> Self {
        Error::DialoguerError(value)
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

impl From<BusinessPlannerError> for Error {
    fn from(value: BusinessPlannerError) -> Self {
        Error::BusinessPlannerError(value)
    }
}

impl From<ClapError> for Error {
    fn from(value: ClapError) -> Self {
        Error::ClapError(value)
    }
}

impl From<InquireError> for Error {
    fn from(value: InquireError) -> Self {
        Error::InquireError(value)
    }
}

impl From<UuidError> for Error {
    fn from(value: UuidError) -> Self {
        Error::UuidError(value)
    }
}

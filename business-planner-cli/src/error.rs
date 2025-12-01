use std::fmt;

use business_planner::api::error::BusinessPlannerError;
pub use clap::Error as ClapError;
pub use inquire::InquireError;
pub use uuid::Error as UuidError;

pub enum NonError {
    Continue,
    Exit,
}

pub enum Error {
    InvalidInput,
    UserCancelled,
    BusinessPlannerError(BusinessPlannerError),
    ClapError(ClapError),
    InquireError(InquireError),
    UuidError(UuidError),
    ErroneousShlexInput,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput => write!(f, "Input is invalid"),
            Error::UserCancelled => write!(f, "Cancelled"),
            Error::BusinessPlannerError(business_planner_error) => write!(f, "{}", business_planner_error),
            Error::ClapError(clap_error) => write!(f, "{clap_error}"),
            Error::InquireError(inquire_error) => write!(f, "{inquire_error}"),
            Error::UuidError(uuid_error) => write!(f, "{uuid_error}"),
            Error::ErroneousShlexInput => write!(f, "Erroneous input provided to command splitter"),
        }
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
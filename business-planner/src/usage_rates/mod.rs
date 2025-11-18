use bigdecimal::ParseBigDecimalError;
use jiff::Timestamp;
use std::{error::Error, fmt::{self, Display, Formatter}};

use crate::registry::structs::{Amount, UsageData};

pub trait Predictor {
    fn time_at_minimum_threshold(&self, minimum_threshold: &Amount) -> Result<Timestamp, PredictionError>;

    fn display(&self) -> Box<dyn Display>;
}

impl fmt::Display for dyn Predictor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.display())
    }
}

pub enum PredictorEstimationError {
    NoUsageData(String),
    ParseBigDecimalError(ParseBigDecimalError),
    BigDecimalCreationError(String),
    EstimatorError(Box<dyn Error>),
}
pub trait Model {
    fn estimate_movement(&self, usage_data: &UsageData) -> Result<Box<dyn Predictor>, PredictorEstimationError>;
}

pub enum PredictionError {
    PredictorError(Box<dyn Error>),
}
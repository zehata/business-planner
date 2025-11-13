use bigdecimal::ParseBigDecimalError;
use crate::structs::{Predictor, UsageData};
use std::error::Error;

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
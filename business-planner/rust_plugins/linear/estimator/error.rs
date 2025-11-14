use std::{fmt::Display, error::Error};

use bigdecimal::{ParseBigDecimalError};
use business_planner::usage_rates::PredictorEstimationError;

#[derive(Debug)]
pub enum LinearPredictorEstimationError {
    ParseBigDecimalError(ParseBigDecimalError)
}

impl Display for LinearPredictorEstimationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", dbg!(self))
    }
}

impl Error for LinearPredictorEstimationError {
    
}

impl From<ParseBigDecimalError> for LinearPredictorEstimationError {
    fn from(value: ParseBigDecimalError) -> Self {
        LinearPredictorEstimationError::ParseBigDecimalError(value)
    }
}

impl From<LinearPredictorEstimationError> for PredictorEstimationError {
    fn from(value: LinearPredictorEstimationError) -> Self {
        PredictorEstimationError::EstimatorError(Box::new(value))
    }
}
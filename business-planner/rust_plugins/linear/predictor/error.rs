use std::{fmt::Display, error::Error};

use business_planner::usage_rates::PredictionError;

#[derive(Debug)]
pub enum LinearPredictionError {
    JiffError(jiff::Error)
}

impl Display for LinearPredictionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", dbg!(self))
    }
}

impl Error for LinearPredictionError {
    
}

impl From<jiff::Error> for LinearPredictionError {
    fn from(value: jiff::Error) -> Self {
        LinearPredictionError::JiffError(value)
    }
}

impl From<LinearPredictionError> for PredictionError {
    fn from(value: LinearPredictionError) -> Self {
        PredictionError::PredictorError(Box::new(value))
    }
}

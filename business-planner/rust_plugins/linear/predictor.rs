use std::{fmt, str::FromStr};

use bigdecimal::BigDecimal;
use business_planner::{structs::{Amount, Predictor}, usage_rates::PredictionError};
use jiff::Timestamp;

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


pub struct LinearPredictor {
    pub m: BigDecimal,
    pub c: BigDecimal,
}

fn calculate_time(predictor: &LinearPredictor, minimum_threshold: &Amount) -> Result<Timestamp, error::LinearPredictionError> {
    let time_string = ((minimum_threshold - &predictor.c)/&predictor.m).to_string();
    Ok(Timestamp::from_str(&time_string)?)
}

impl Predictor for LinearPredictor {
    fn time_at_minimum_threshold(&self, minimum_threshold: &Amount) -> Result<Timestamp, PredictionError> {
        Ok(calculate_time(self, minimum_threshold)?)
    }

    fn display(&self) -> Box<dyn fmt::Display> {
        Box::new(format!("y = {}x + {}", self.m, self.c))
    }
}
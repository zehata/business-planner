use std::{fmt, str::FromStr};

use bigdecimal::BigDecimal;
use business_planner::{structs::{Amount, Predictor}, usage_rates::PredictionError};
use jiff::Timestamp;

pub mod error;

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
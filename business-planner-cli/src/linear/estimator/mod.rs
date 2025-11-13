use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive, ParseBigDecimalError};
use business_planner::{structs::{Predictor, StockLevel, UsageData}, usage_rates::{Model, PredictorEstimationError}};

use crate::linear::{estimator::error::LinearPredictorEstimationError, predictor::LinearPredictor};

pub mod error;

pub struct LinearRegression {

}

struct UsageDataByAxes {
    amounts: Vec<BigDecimal>,
    timestamps: Vec<BigDecimal>,
}

fn transpose_usage_data (usage_data: &UsageData) -> Result<UsageDataByAxes, LinearPredictorEstimationError> {
    let no_of_points = usage_data.stock_levels.len();
    Ok(usage_data.stock_levels.clone().into_iter().try_fold(
        UsageDataByAxes{
            amounts: Vec::with_capacity(no_of_points),
            timestamps: Vec::with_capacity(no_of_points)
        },
        |mut accumulated, StockLevel{amount, timestamp}| -> Result<UsageDataByAxes, ParseBigDecimalError> {
            accumulated.amounts.push(amount);
            let timestamp_in_decimal = BigDecimal::from_str(&timestamp.to_string())?;
            accumulated.timestamps.push(timestamp_in_decimal);
            Ok(accumulated)
        }
    )?)
}

impl Model for LinearRegression {
    fn estimate_movement(&self, usage_data: &UsageData) -> Result<Box<dyn Predictor>, PredictorEstimationError> {
        if usage_data.stock_levels.is_empty() {
            return Err(PredictorEstimationError::NoUsageData("No usage data found".to_string()))
        }

        let usage_data_by_axes = transpose_usage_data(usage_data)?;

        let Some(len_x) = bigdecimal::BigDecimal::from_usize(usage_data_by_axes.amounts.len()) else {
            return Err(PredictorEstimationError::NoUsageData("No usage data found".to_string()))
        };

        let Some(len_y) = bigdecimal::BigDecimal::from_usize(usage_data_by_axes.timestamps.len()) else {
            return Err(PredictorEstimationError::NoUsageData("No usage data found".to_string()))
        };
        let mean_y = usage_data_by_axes.amounts.iter().sum::<BigDecimal>() / len_x;
        let mean_x = usage_data_by_axes.timestamps.iter().sum::<BigDecimal>() / len_y;
        let errors_x = usage_data_by_axes.amounts.iter().map(|x| { x - &mean_x}).collect::<Vec<BigDecimal>>();
        let errors_y = usage_data_by_axes.amounts.iter().map(|y| { y - &mean_x}).collect::<Vec<BigDecimal>>();
        
        let Some(zero) = BigDecimal::from_i32(0) else {
            return Err(PredictorEstimationError::BigDecimalCreationError("Failed to create a BigDecimal of value 0".to_string()))
        };

        let nominator = errors_x.iter().zip(errors_y).fold(
            zero.clone(),
            |mut accumulated, (error_x, error_y)| {
                accumulated += error_x * error_y;
                accumulated
            }
        );
        let denominator = errors_x.iter().fold(
            zero,
            |mut accumulated, error_x| {
                accumulated += error_x * error_x;
                accumulated
            }
        );
        let m = nominator / denominator;
        
        let c = mean_y - &m * mean_x;

        Ok(Box::new(LinearPredictor{m, c}))
    }
}
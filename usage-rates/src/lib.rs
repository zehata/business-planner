use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive, ParseBigDecimalError};
use jiff::Timestamp;

type Amount = BigDecimal;

#[derive(Clone)]
pub struct StockLevel {
    pub amount: Amount,
    pub timestamp: Timestamp,
}

pub struct TargetWindow {
    pub target: Amount,
    pub window: Amount,
}

pub struct Thresholds {
    pub minimum: Amount,
    pub maximum: Amount,
}

pub enum StockLevelTarget {
    TargetWindow{target: Amount, upward_window: Amount, downward_window: Amount},
    Thresholds{minimum: Amount, maximum: Amount},
}

pub struct UsageData {
    stock_levels: Vec<StockLevel>
}

pub trait Predictor {
    fn time_at_minimum_threshold(&self, minimum_threshold: &Amount) -> Result<Timestamp, jiff::Error>;
}

struct LinearPredictor {
    m: BigDecimal,
    c: BigDecimal,
}

impl Predictor for LinearPredictor {
    fn time_at_minimum_threshold(&self, minimum_threshold: &Amount) -> Result<Timestamp, jiff::Error> {
        let time_string = ((minimum_threshold - &self.c)/&self.m).to_string();
        Timestamp::from_str(&time_string)
    }
}

pub trait Model {
    fn estimate_movement(&self, usage_data: &UsageData) -> Result<Box<dyn Predictor>, PredictorEstimationError>;
}

struct LinearRegression {

}

impl Model for LinearRegression {
    fn estimate_movement(&self, usage_data: &UsageData) -> Result<Box<dyn Predictor>, PredictorEstimationError> {
        if usage_data.stock_levels.is_empty() {
            return Err(PredictorEstimationError::NoUsageData("No usage data found".to_string()))
        }

        struct UsageDataByAxes {
            amounts: Vec<BigDecimal>,
            timestamps: Vec<BigDecimal>,
        }

        let no_of_points = usage_data.stock_levels.len();
        let usage_data_by_axes = usage_data.stock_levels.clone().into_iter().try_fold(
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
        )?;

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

const DEFAULT_PREDICTION_MODEL: LinearRegression = LinearRegression{};

pub enum PredictorEstimationError {
    NoUsageData(String),
    ParseBigDecimalError(ParseBigDecimalError),
    BigDecimalCreationError(String),
}

impl From<ParseBigDecimalError> for PredictorEstimationError {
    fn from(value: ParseBigDecimalError) -> Self {
        PredictorEstimationError::ParseBigDecimalError(value)
    }
}

pub enum StockTakePredictionError {
    PredictorEstimationError(PredictorEstimationError),
    TimestampConversionError(jiff::Error),
}

impl From<PredictorEstimationError> for StockTakePredictionError {
    fn from(value: PredictorEstimationError) -> Self {
        StockTakePredictionError::PredictorEstimationError(value)
    }
}

impl From<jiff::Error> for StockTakePredictionError {
    fn from(value: jiff::Error) -> Self {
        StockTakePredictionError::TimestampConversionError(value)
    }
}

pub fn predict_next_stock_take(usage_data: &UsageData, target: &StockLevelTarget, prediction_model: &Option<Box<dyn Model>>) -> Result<Timestamp, StockTakePredictionError> {
    let model = match prediction_model {
        Some(model) => model,
        _ => &(Box::new(DEFAULT_PREDICTION_MODEL) as Box<dyn Model>)
    };
    let predictor = model.estimate_movement(usage_data)?;
    let minimum_threshold = match target {
        StockLevelTarget::TargetWindow{target, downward_window, upward_window: _} => {
            target - downward_window
        },
        StockLevelTarget::Thresholds{minimum, maximum: _} => minimum.clone(),
    };
    Ok(predictor.time_at_minimum_threshold(&minimum_threshold)?)
}
use business_planner::{reports::error::ReportGenerationError, structs::{Amount, Report, StockLevelTarget, Store}, usage_rates::Model};

use crate::linear::estimator::LinearRegression;

pub fn generate(store: &Store) -> Result<Report, ReportGenerationError> {
    let target = &StockLevelTarget::Thresholds{
        maximum: Amount::from(100),
        minimum: Amount::from(10),
    };
    
    let predictor = LinearRegression{}.estimate_movement(&store.usage_data)?;

    let minimum_threshold = match target {
        StockLevelTarget::TargetWindow{target, downward_window, upward_window: _} => {
            target - downward_window
        },
        StockLevelTarget::Thresholds{minimum, maximum: _} => minimum.clone(),
    };
    let time = predictor.time_at_minimum_threshold(&minimum_threshold)?;
    
    Ok(format!("Next stock take prediction: {}\nEstimated predictor: {}\nEstimator: Ordinary linear regression", time, predictor))
}
use crate::usage_rates::{PredictionError, PredictorEstimationError};

pub enum ReportGenerationError {
    PredictorEstimationError(PredictorEstimationError),
    PredictionError(PredictionError),
}

impl From<PredictorEstimationError> for ReportGenerationError {
    fn from(value: PredictorEstimationError) -> Self {
        ReportGenerationError::PredictorEstimationError(value)
    }
}

impl From<PredictionError> for ReportGenerationError {
    fn from(value: PredictionError) -> Self {
        ReportGenerationError::PredictionError(value)
    }
}
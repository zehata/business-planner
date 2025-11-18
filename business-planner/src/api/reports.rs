use crate::reports::{self, Report};

pub use reports::error::ReportGenerationError;

pub fn generate_report(plugin_name: String) -> Result<Report, ReportGenerationError> {
    reports::generate_report(plugin_name)
}
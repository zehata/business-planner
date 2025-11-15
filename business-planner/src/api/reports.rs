use crate::reports;

pub use reports::error::ReportGenerationError;

pub fn generate_report(plugin_name: String) -> Result<String, ReportGenerationError> {
    reports::generate_report(plugin_name)
}
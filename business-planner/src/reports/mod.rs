use crate::{plugins::{get_plugins, python::run_script_unsandboxed}, reports::error::{PluginNotFound, ReportGenerationError}, sandbox::create_sandbox};

pub mod error;

pub fn generate_report (plugin_name: String) -> Result<String, ReportGenerationError> {
    let plugins = get_plugins()?;
    let Some(plugin) = plugins.get(&plugin_name) else {
        return Err(ReportGenerationError::PluginNotFound(PluginNotFound::new(plugin_name)))
    };
    let script_path = &plugin.generate_report_script;
    create_sandbox();
    Ok(run_script_unsandboxed(script_path)?)
}
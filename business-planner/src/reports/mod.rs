use crate::{plugins::{get_plugins, run_script}, reports::error::{PluginNotFound, ReportGenerationError}};

pub mod error;

pub type Report = String;

pub fn generate_report (plugin_name: String) -> Result<Report, ReportGenerationError> {
    let plugins = get_plugins()?;
    let Some(plugin) = plugins.get(&plugin_name) else {
        return Err(ReportGenerationError::PluginNotFound(PluginNotFound::new(plugin_name)))
    };
    let script_output = run_script(plugin)?.stdout;
    Ok(String::from_utf8(script_output)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn report_test() {
        let result = generate_report("linear".to_string());
        println!("{:#?}", result)
    }
}
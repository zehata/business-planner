use std::{fs, path::Path};
use pyo3::{prelude::*, types::PyModule};
use std::ffi::CString;

use crate::plugins::error::PluginError;

pub fn run_script_unsandboxed (path: &Path) -> Result<String, PluginError> {
    let code = fs::read_to_string(path)?;

    Python::attach(|py| -> Result<String, PluginError> {
        let module = PyModule::from_code(py, CString::new(code)?.as_c_str(), c"test.py", c"test")?;
        let result = module.call_method0("test_func")?;
        
        Ok(result.to_string())
    })
}
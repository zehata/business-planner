use std::{fs, path::Path};
use pyo3::{prelude::*, types::PyModule};
use std::ffi::CString;

use crate::plugins_api::error::PluginError;

pub fn test_func () -> Result<String, PluginError> {
    let path = Path::new("./plugins/test.py");
    let code = fs::read_to_string(path)?;

    Python::attach(|py| -> Result<String, PluginError> {
        let module = PyModule::from_code(py, CString::new(code)?.as_c_str(), c"test.py", c"test")?;
        let result = module.call_method0("test_func")?;
        
        Ok(result.to_string())
    })
}

pub fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        if let Ok(result) = test_func() {
            print!("{:#?}", result);
        }
    }
}

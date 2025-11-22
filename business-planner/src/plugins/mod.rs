use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::path::{PathBuf, absolute};
use std::process::{Command, Output};

use crate::plugins::error::{PluginError, PluginDiscoveryError};

pub mod error;

#[derive(Debug)]
enum PluginType {
    Python,
    Binary,
}

#[derive(Debug)]
pub struct Plugin {
    path: PathBuf,
    plugin_type: PluginType,
}

pub enum DirectoryEntriesFilter {
    File,
    Directory,
}

fn list_directory_contents (path: &PathBuf, filter: Option<DirectoryEntriesFilter>) -> Result<Vec<PathBuf>, error::PluginDiscoveryError> {
    let directory_entries = fs::read_dir(path)?;
    Ok(directory_entries.filter_map(|dir_entry| {
        let path = match dir_entry {
            Ok(path) => path,
            Err(error) => return Some(Err(error))
        };

        if let Some(filter) = &filter {
            let file_type = match path.file_type() {
                Ok(file_type) => file_type,
                Err(error) => return Some(Err(error)),
            };

            match filter {
                DirectoryEntriesFilter::File => {
                    if !file_type.is_file() {
                        return None
                    }
                },
                DirectoryEntriesFilter::Directory => {
                    if !file_type.is_dir() {
                        return None
                    }
                }
            }
        }

        Some(Ok(path.path()))
    }).collect::<Result<Vec<PathBuf>, _>>()?)
}

static REQUIRED_FILES: [&str; 1] = ["main"];

pub fn get_plugins () -> Result<HashMap<String, Plugin>, error::PluginDiscoveryError> {
    let current_directory = PathBuf::from("./plugins/");
    let directory_contents = list_directory_contents(&current_directory, Some(DirectoryEntriesFilter::Directory))?;

    let mut plugins = HashMap::new();
    
    directory_contents.into_iter().try_for_each(|directory| {
        let Ok(files) = list_directory_contents(&directory, Some(DirectoryEntriesFilter::File)) else {
            return Err(PluginDiscoveryError::ReadDirectoryError)
        };

        let file_names = files.iter().filter_map(|file| {
            let file_name = file.file_prefix()?;
            
            file_name.to_str()
        });

        let file_names = HashSet::from_iter(file_names);
        let required_file_names = HashSet::from(REQUIRED_FILES);

        if file_names.intersection(&required_file_names).count() != REQUIRED_FILES.len() {
            return Ok(())
        }

        let Some(plugin_directory_name) = directory.file_name() else {
            return Err(PluginDiscoveryError::ReadDirectoryError)
        };

        let Ok(plugin_directory_name) = plugin_directory_name.to_os_string().into_string() else {
            return Err(PluginDiscoveryError::ReadDirectoryError)
        };

        plugins.insert(plugin_directory_name, Plugin{
            path: {
                let mut script_path = directory;
                script_path.push("main.py");
                script_path
            },
            plugin_type: PluginType::Python,
        });

        Ok(())
    })?;

    Ok(plugins)
}

pub fn run_script (plugin: &Plugin) -> Result<Output, PluginError> {
    let absolute_path = absolute(plugin.path.clone())?;
    let absolute_path = absolute_path.as_os_str();
    Ok(match plugin.plugin_type {
        PluginType::Python => {
            Command::new("python3").args([absolute_path]).output()
        },
        PluginType::Binary => {
            Command::new(absolute_path).output()
        },
    }?)
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_func() {
//         let result = run_script_unsandboxed(&Plugin { path: PathBuf::from("./plugins/linear/main.py"), plugin_type: PluginType::Python });
//         println!("{:#?}", result);
//     }
// }
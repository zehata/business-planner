use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::io::{BufRead, BufReader, Write};
use std::path::{PathBuf, absolute};
use std::process::{Command, Stdio};

use crate::error::Error;
use crate::plugins::error::{PluginError, PluginDiscoveryError};

pub mod error;

#[derive(Debug, Clone)]
enum PluginType {
    Python,
    Binary,
}

#[derive(Debug, Clone)]
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

pub fn run_script (plugin: &Plugin) -> Result<String, PluginError> {
    let absolute_path = absolute(plugin.path.clone())?;
    let Some(parent) = absolute_path.parent() else {
        return Err(PluginError::PluginMissingError)
    };
    match plugin.plugin_type {
        PluginType::Python => {
            let mut command = Command::new("./.venv/bin/python");
            command.current_dir(parent);
            command.args([absolute_path]);
            command.stdin(Stdio::piped());
            command.stdout(Stdio::piped());
            let mut child_process = command.spawn()?;

            let stdin = child_process.stdin.as_mut().unwrap();
            let mut stdout = BufReader::new(child_process.stdout.as_mut().unwrap()).lines();
            
            let mut buf = String::new();
            while let Some(Ok(response)) = stdout.next() {
                if &response[..] == "request_data" {
                    println!("Sending data!");
                    stdin.write_all("some_data\n".as_bytes()).unwrap();
                }

                buf = response;
                println!("{buf}");

                if &buf[..] == "complete" {
                    break
                }
            };

            child_process.wait()?;
            Ok(buf)
        },
        PluginType::Binary => {
            unimplemented!()
        },
    }
}

pub fn run_plugin (plugin_name: &str) -> Result<String, Error> {
    let plugins = get_plugins()?;
    let Some(plugin) = plugins.get(plugin_name) else {
        return Err(Error::PluginDiscoveryError(PluginDiscoveryError::PluginNotFound))
    };
    Ok(run_script(plugin)?)
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
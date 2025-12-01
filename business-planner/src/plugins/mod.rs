use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::io::{self, BufRead, BufReader, Lines, Write};
use std::path::{PathBuf, absolute};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use serde::Serialize;

use sonic_rs::{Serializer, json};

use crate::error::Error;
use crate::plugins::error::{PluginError, PluginDiscoveryError};
use crate::registry::RegistryItem;

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

pub enum PluginResponse {
    DataRequest(String),
    Message(String),
    Report(String),
    ProcessEnded,
}

pub struct PluginResponses {
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl PluginResponses {
    pub fn try_next(&mut self) -> Option<Result<PluginResponse, Error>> {
        let result = self.stdout.next()?;

        let result = match result {
            Ok(result) => result,
            Err(error) => return Some(Err(Error::PluginError(PluginError::IoError(error)))),
        };

        Some(Ok(match &result[..] {
            "request_data" => PluginResponse::DataRequest("Data requested".to_string()),
            "report" => {
                let report = self.stdout.next()?;

                let report = match report {
                    Ok(report) => report,
                    Err(error) => return Some(Err(Error::PluginError(PluginError::IoError(error)))),
                };

                PluginResponse::Report(report)
            },
            _ => PluginResponse::Message(result),
        }))
    }
}

pub struct PluginProcess {
    child_process: Child,
    pub responses: PluginResponses,
}

impl PluginProcess {
    fn new (mut child_process: Child) -> PluginProcess {
        let stdin = child_process.stdin.take().unwrap();
        let stdout = BufReader::new(child_process.stdout.take().unwrap()).lines();
        let responses = PluginResponses {
            stdin,
            stdout,
        };

        PluginProcess {
            child_process,
            responses,
        }
    }

    pub fn send_response<T: RegistryItem>(&mut self, item: &mut T) {
        let mut ser = Serializer::new(Vec::new());
        let value = json!(item);
        value.serialize(&mut ser).unwrap();
        let mut bytes = ser.into_inner();
        bytes.push(b'\n');
        let stdin = &mut self.responses.stdin;
        stdin.write_all(&bytes).unwrap();
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.child_process.wait()
    }
}

pub fn run_script (plugin: &Plugin) -> Result<PluginProcess, PluginError> {
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

            let child_process = command.spawn()?;            
            let plugin_process = PluginProcess::new(child_process);

            Ok(plugin_process)
        },
        PluginType::Binary => {
            unimplemented!()
        },
    }
}

pub fn run_plugin (plugin_name: &str) -> Result<PluginProcess, Error> {
    let plugins = get_plugins()?;
    let Some(plugin) = plugins.get(plugin_name) else {
        return Err(Error::PluginDiscoveryError(PluginDiscoveryError::PluginNotFound))
    };
    Ok(run_script(plugin)?)
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use serde::Serialize;
//     use sonic_rs::{Deserializer, Serializer, json};

//     #[test]
//     fn test_func() {
//         let mut ser = Serializer::new(Vec::new());
//         let value = json!(Material::new("test"));
//         value.serialize(&mut ser).unwrap();
//         let json_str = String::from_utf8(ser.into_inner()).unwrap();
//         println!("{:?}", json_str);
        
//         let mut deser = Deserializer::from_str(&json_str);
//         let material: Material = deser.deserialize().unwrap();
//         println!("{:?}", material);
//     }
// }
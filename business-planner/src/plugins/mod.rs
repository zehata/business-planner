use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::{PathBuf, absolute};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::str::FromStr;
use copy_dir::copy_dir;
use serde::{Deserialize, Serialize};

use sonic_rs::{Deserializer, Serializer, json};

use crate::api::registry::{Material, Store};
use crate::error::Error;
use crate::io::error::IoError;
use crate::plugins::error::{PluginError, PluginManagementError};
use crate::registry::structs::store::StoreData;
use crate::registry::{RegistryItem, RegistryItemInternals};

pub mod error;

#[derive(Serialize, Deserialize, PartialEq)]
enum PluginType {
    Python,
    Binary,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Plugin {
    path: PathBuf,
    plugin_type: PluginType,
}

pub enum DirectoryEntriesFilter {
    File,
    Directory,
}

fn list_directory_contents (path: &PathBuf, filter: Option<DirectoryEntriesFilter>) -> Result<Vec<PathBuf>, error::PluginManagementError> {
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

pub fn list_plugins () -> Result<Vec<String>, error::PluginManagementError> {
    let plugins = get_plugins()?;
    Ok(plugins.keys().cloned().collect())
}

pub fn get_plugins () -> Result<HashMap<String, Plugin>, error::PluginManagementError> {
    let current_directory = PathBuf::from("./plugins/");
    let directory_contents = list_directory_contents(&current_directory, Some(DirectoryEntriesFilter::Directory))?;

    let mut plugins = HashMap::new();
    
    directory_contents.into_iter().try_for_each(|directory| {
        let Ok(files) = list_directory_contents(&directory, Some(DirectoryEntriesFilter::File)) else {
            return Err(PluginManagementError::ReadDirectoryError)
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

        let plugin_directory_name = directory.file_name().ok_or(PluginManagementError::ReadDirectoryError)?;

        let Ok(plugin_directory_name) = plugin_directory_name.to_os_string().into_string() else {
            return Err(PluginManagementError::ReadDirectoryError)
        };

        plugins.insert(plugin_directory_name, Plugin{
            path: directory,
            plugin_type: PluginType::Python,
        });

        Ok(())
    })?;

    Ok(plugins)
}

pub struct AnyDataRequest<'a> {
    stdin: &'a mut ChildStdin,
}

impl DataRequest<Material> for AnyDataRequest<'_> {
    fn get_stdin(&mut self) -> &mut ChildStdin {
        self.stdin
    }

    fn format_message(&self, item: &Material) -> Result<Response, Error> {
        Ok(Response::DataResponse(DataResponse::Material(item.clone())))
    }
}

impl DataRequest<Store> for AnyDataRequest<'_> {
    fn get_stdin(&mut self) -> &mut ChildStdin {
        self.stdin
    }

    fn format_message(&self, item: &Store) -> Result<Response, Error> {
        match item.fetch_data() {
            Ok(store) => Ok(Response::DataResponse(DataResponse::Store(store))),
            Err(error) => Err(Error::IoError(IoError::ReadError(error)))
        }
    }
}

pub struct MaterialDataRequest<'a> {
    stdin: &'a mut ChildStdin,
}

impl DataRequest<Material> for MaterialDataRequest<'_> {
    fn get_stdin(&mut self) -> &mut ChildStdin {
        self.stdin
    }

    fn format_message(&self, item: &Material) -> Result<Response, Error> {
        match item.fetch_data() {
            Ok(material) => Ok(Response::DataResponse(DataResponse::Material(material))),
            Err(error) => Err(Error::IoError(IoError::ReadError(error)))
        }
    }
}

pub trait DataRequest<T: RegistryItem> {
    fn get_stdin(&mut self) -> &mut ChildStdin;

    fn format_message(&self, item: &T) -> Result<Response, Error>;

    fn send_response(&mut self, item: &T) -> Result<(), Error> {
        let message = self.format_message(item)?;

        let value = json!(message);
        let mut ser = Serializer::new(Vec::new());
        value.serialize(&mut ser).unwrap();

        let mut bytes = ser.into_inner();
        bytes.push(b'\n');
        
        let stdin = self.get_stdin();
        stdin.write_all(&bytes).unwrap();
        Ok(())
    }
}

pub enum PluginResponse<'a> {
    AnyDataRequest(AnyDataRequest<'a>),
    MaterialDataRequest(MaterialDataRequest<'a>),
    Message(String),
    Report(String),
    ProcessEnded,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataResponse {
    Material(Material),
    Store(StoreData),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    DataResponse(DataResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    DataRequest,
}

pub struct PluginResponses {
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl PluginResponses {
    pub fn try_next(&mut self) -> Option<Result<PluginResponse<'_>, Error>> {
        let result = self.stdout.next()?;

        let result = match result {
            Ok(result) => result,
            Err(error) => return Some(Err(Error::PluginError(PluginError::IoError(error)))),
        };

        let mut deser = Deserializer::from_str(&result);
        let Ok(message) = deser.deserialize() else {
            return Some(Ok(PluginResponse::Message(result)))
        };

        let plugin_response = match message {
            Message::DataRequest => {
                PluginResponse::AnyDataRequest(AnyDataRequest { stdin: &mut self.stdin })
            },
        };

        Some(Ok(plugin_response))
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
    
    pub fn await_exit(&mut self) -> Result<ExitStatus, Error> {
        match self.child_process.wait() {
            Ok(exit_status) => Ok(exit_status),
            Err(error) => Err(Error::PluginError(PluginError::IoError(error))),
        }
    }
}

pub fn run_script (plugin: &Plugin) -> Result<PluginProcess, PluginError> {
    let absolute_path = absolute(plugin.path.clone())?;
    // let parent = absolute_path.parent().ok_or(PluginError::PluginMissingError)?;
    match plugin.plugin_type {
        PluginType::Python => {
            let mut command = Command::new("./.venv/bin/python");
            command.current_dir(absolute_path);
            command.args(["main.py"]);
            command.stdin(Stdio::piped());
            command.stdout(Stdio::piped());

            let child_process = command.spawn()?;            
            let plugin_process = PluginProcess::new(child_process);

            Ok(plugin_process)
        },
        PluginType::Binary => {
            todo!()
        },
    }
}

pub fn get_plugin(plugin_name: &str) -> Result<Plugin, PluginManagementError> {
    let mut plugins = get_plugins()?;
    plugins.remove(plugin_name).ok_or(PluginManagementError::PluginNotFound)
}

pub fn run_plugin(plugin_name: &str) -> Result<PluginProcess, Error> {
    let plugin = get_plugin(plugin_name)?;
    Ok(run_script(&plugin)?)
}

pub fn remove_plugin(plugin_name: &str) -> Result<(), Error> {
    let plugin = get_plugin(plugin_name)?;
    if let Err(error) = trash::delete(plugin.path) {
        return Err(PluginManagementError::TrashError(error))?
    }
    Ok(())
}

pub fn add_plugin(path: &PathBuf) -> Result<(), PluginManagementError> {
    let mut dest = PathBuf::from_str("./plugins/").unwrap();
    dest.extend(path.file_name());
    copy_dir(path, dest)?;
    Ok(())
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
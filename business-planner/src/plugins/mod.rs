use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::path::{PathBuf};

pub mod python;
pub mod error;

pub struct Plugin {
    pub generate_report_script: PathBuf,
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

pub fn get_plugins () -> Result<HashMap<String, Plugin>, error::PluginDiscoveryError> {
    let current_directory = PathBuf::from("./plugins/");
    let directory_contents = list_directory_contents(&current_directory, Some(DirectoryEntriesFilter::Directory))?;

    let mut plugins = HashMap::new();
    
    directory_contents.into_iter().for_each(|directory| {
        let Ok(files) = list_directory_contents(&directory, Some(DirectoryEntriesFilter::File)) else {
            return
        };

        let file_names = files.iter().filter_map(|file| {
            let file_name = file.file_name()?;
            
            file_name.to_str()
        });

        let file_names = HashSet::from_iter(file_names);

        let required_file_names = HashSet::from(["estimate_predictor.py", "use_predictor.py", "generate_report.py"]);

        if file_names.intersection(&required_file_names).count() != 3 {
            return
        }

        let Some(plugin_directory_name) = directory.to_str() else {
            return
        };

        plugins.insert(plugin_directory_name.to_string(), Plugin{
            generate_report_script: {
                let mut script_path = directory.clone();
                script_path.push("generate_report.py");
                script_path
            },
        });
    });

    Ok(plugins)
}
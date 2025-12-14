use std::{env::{self, set_current_dir}, fs, path::PathBuf};

use inquire::Select;

use crate::Error;

pub async fn select_file() -> Result<PathBuf, Error> {
    let mut file_path: Option<PathBuf> = None;
    while file_path.is_none() {
        let selected_path: PathBuf =  prompt_select_from_current_dir()?;

        if selected_path.is_dir() {
            set_current_dir(selected_path)?;
            continue
        }

        if selected_path.is_file() {
            file_path = Some(selected_path)
        }
    }
    Ok(file_path.expect("Path selection loop to end only when path has been selected"))
}

fn prompt_select_from_current_dir() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir().unwrap();
    let files = fs::read_dir(&current_dir)?;
    
    let mut valid_dir_entries = files.filter_map(|file| {
        let Ok(file) = file else {
            return None
        };

        let path = file.path();
        let name = path.file_name()?.to_str()?.to_string();
        
        if path.is_dir() {
            return Some((path, format!("{}/", name)))
        }

        if path.is_file() && path.extension()?.to_str()? == "xlsx" {
            return Some((path, name))
        }

        None
    }).collect::<Vec<_>>();

    valid_dir_entries.sort_by(|a, b| { a.1.cmp(&b.1) });
    
    let mut valid_paths = match current_dir.parent() {
        Some(parent_dir) => vec![(parent_dir.to_path_buf(), "..".to_string())],
        None => vec![],
    };

    valid_paths.extend(valid_dir_entries);

    let valid_path_names = valid_paths.iter().map(|path| {path.1.clone()}).collect::<Vec<_>>();

    let selection = Select::new("Select file", valid_path_names).raw_prompt()?;

    Ok(valid_paths.get(selection.index).unwrap().0.clone())
}

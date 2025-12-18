use std::{fs, path::{Path, PathBuf}};

use inquire::Select;

use crate::Error;

pub enum PathFilter<'a> {
    File(Option<&'a str>),
    Directory,
}

fn get_valid_paths(path: &PathBuf, path_filter: &PathFilter) -> Result<Vec<(PathBuf, String)>, Error> {
    let files = fs::read_dir(path)?;
    
    let mut valid_dir_entries = files.filter_map(|file| {
        let Ok(file) = file else {
            return None
        };

        let path = file.path();
        let name = file.file_name().to_str()?.to_string();

        if path.is_dir() {
            return Some((path, format!("{}/", name)))
        }

        if path.is_file() && let PathFilter::File(extension) = *path_filter {
            match extension {
                Some(extension) => {
                    match path.extension()?.to_str()?.eq(extension) {
                        true => return Some((path, name)),
                        false => return None
                    }
                },
                None => return Some((path, name))
            }
        }

        None
    }).collect::<Vec<_>>();

    valid_dir_entries.sort_by(|a, b| { a.1.cmp(&b.1) });
    
    let mut valid_paths = match path.parent() {
        Some(parent_dir) => vec![(parent_dir.to_path_buf(), "..".to_string())],
        None => vec![],
    };

    valid_paths.extend(valid_dir_entries);

    if let PathFilter::Directory = path_filter {
        valid_paths.extend(vec![(path.clone(), "Select current directory".to_string())])
    }

    Ok(valid_paths)
}

pub async fn prompt_select_path(path: &Path, path_filter: PathFilter<'_>) -> Result<PathBuf, Error> {
    let mut current_dir = path.to_path_buf();
    let mut file_path: Option<PathBuf> = None;
    while file_path.is_none() {
        let mut valid_paths = get_valid_paths(&current_dir, &path_filter)?;
        let valid_path_names = valid_paths.iter().map(|path| {path.1.clone()}).collect::<Vec<_>>();

        let selection = Select::new("Select file", valid_path_names).raw_prompt()?;

        let selected_path = valid_paths.swap_remove(selection.index).0;

        if selected_path.is_dir() {
            match &selected_path.eq(&current_dir) {
                true => {
                    file_path = Some(selected_path);
                },
                false => {
                    current_dir = selected_path;
                    continue
                },
            }
        }
    }
    Ok(file_path.expect("Path selection loop to end only when path has been selected"))
}
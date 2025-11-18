use std::{fmt, path::{Path, PathBuf}};
use business_planner::api::session::{Session, save_to_last_save_location, save_to_location};
use inquire::Text;
use clap::Subcommand;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::error;

#[derive(Debug, Subcommand, EnumIter)]
pub enum Command {
    /// Adds a new item to the registry
    #[command()]
    Create {
        /// Item type
        #[arg()]
        path: Option<PathBuf>,
    },

    /// Read registry info on an item
    #[command()]
    Read {
        /// Path to save file to
        #[arg()]
        path: Option<PathBuf>,
    },
    
    /// Update an item in the registry
    #[command()]
    Update {
        /// Path to save file to
        #[arg()]
        path: Option<PathBuf>,
    },
    
    /// Removes an item from the registry
    #[command()]
    Remove {
        /// Path to save file to
        #[arg()]
        path: Option<PathBuf>,
    },
}
mod create;
mod read;
mod update;
mod delete;
mod list;
mod add;
mod remove;
mod plugins;
mod save;

pub use {create::CreateMenu, read::ReadMenu, update::UpdateMenu, delete::DeleteMenu, list::ListMenu, add::AddMenu, remove::RemoveMenu, plugins::PluginsMenu, save::SaveMenu};
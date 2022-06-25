mod expedient_hooks;
mod expedient_modifyer;

pub use expedient_modifyer::*;
pub use expedient_hooks::*;

use crate::{
    database::RollbackDateInfo,
    error::{ErrorKind, Result},
    expedient_database::ExpedientDatabase,
};
use std::{path::PathBuf, sync::Mutex};

#[derive(Default)]
pub struct ApiState<'a> {
    pub database_mutex: Mutex<Option<ExpedientDatabase<'a>>>,
}

#[tauri::command]
pub fn open_database(state: tauri::State<ApiState>, path: PathBuf) -> Result<()> {
    let mut database = state.database_mutex.lock().unwrap();
    if database.is_none() {
        *database = Some(ExpedientDatabase::open(&path)?);
        Ok(())
    } else {
        ErrorKind::AlreadyOpen.into()
    }
}

#[tauri::command]
pub fn create_database(state: tauri::State<ApiState>, path: PathBuf) -> Result<()> {
    let mut database = state.database_mutex.lock().unwrap();
    if database.is_none() {
        *database = Some(ExpedientDatabase::create(&path)?);
        Ok(())
    } else {
        ErrorKind::AlreadyOpen.into()
    }
}

#[tauri::command]
pub fn rollback_database(state: tauri::State<ApiState>, path: PathBuf) -> Result<()> {
    let mut database = state.database_mutex.lock().unwrap();
    if database.is_none() {
        *database = Some(ExpedientDatabase::rollback(&path)?);
        Ok(())
    } else {
        ErrorKind::AlreadyOpen.into()
    }
}

#[tauri::command]
pub fn database_rollback_info(
    state: tauri::State<ApiState>,
    path: PathBuf,
) -> Result<RollbackDateInfo> {
    if state.database_mutex.lock().unwrap().is_none() {
        ExpedientDatabase::rollback_info(&path)
    } else {
        ErrorKind::AlreadyOpen.into()
    }
}

#[tauri::command]
pub fn store_database(state: tauri::State<ApiState>) -> Result<()> {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.store()
    } else {
        ErrorKind::NotFound.into()
    }
}

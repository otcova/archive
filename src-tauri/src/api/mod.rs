mod expedient_hooks;
mod expedient_modifyer;

pub use expedient_hooks::*;
pub use expedient_modifyer::*;

pub use crate::expedient_database::*;
use crate::{
    database::RollbackDateInfo,
    error::{ErrorKind, Result},
};

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Default, Clone)]
pub struct ApiState {
    pub database_mutex: Arc<Mutex<Option<ExpedientDatabase<'static>>>>,
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
pub async fn store_database(state: tauri::State<'_, ApiState>) -> Result<()> {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.save()
    } else {
        ErrorKind::NotFound.into()
    }
}

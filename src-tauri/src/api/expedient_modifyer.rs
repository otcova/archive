use super::ApiState;
use crate::{chunked_database::Uid, expedient_database::*};

#[tauri::command]
pub async fn create_expedient(
    state: tauri::State<'_, ApiState>,
    expedient: Expedient,
) -> Result<Option<Uid>, ()> {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        Ok(Some(database.create_expedient(expedient)))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn delete_expedient(state: tauri::State<ApiState>, id: Uid) {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.delete_expedient(id);
    }
}

#[tauri::command]
pub async fn update_expedient(
    state: tauri::State<'_, ApiState>,
    id: Uid,
    expedient: Expedient,
) -> Result<(), ()> {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.update_expedient(id, expedient);
    }
    Ok(())
}

#[tauri::command]
pub fn read_expedient(state: tauri::State<ApiState>, id: Uid) -> Option<Expedient> {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.read_expedient(id)
    } else {
        None
    }
}

#[tauri::command]
pub fn count_expedients(state: tauri::State<ApiState>) -> usize {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.count_expedients()
    } else {
        0
    }
}

#[tauri::command]
pub fn delete_repeated(state: tauri::State<ApiState>) -> usize {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.delete_repeated()
    } else {
        0
    }
}

#[tauri::command]
pub fn count_orders(state: tauri::State<ApiState>) -> usize {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.count_orders()
    } else {
        0
    }
}

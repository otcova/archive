use super::ApiState;
use crate::{chunked_database::Uid, expedient_database::*};

#[tauri::command]
pub fn create_expedient(state: tauri::State<ApiState>, expedient: Expedient) -> Option<Uid> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(database.as_mut()?.create_expedient(expedient))
}

#[tauri::command]
pub fn delete_expedient(state: tauri::State<ApiState>, id: Uid) {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.delete_expedient(id);
    }
}

#[tauri::command]
pub fn update_expedient(state: tauri::State<ApiState>, id: Uid, expedient: Expedient) {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.update_expedient(id, expedient);
    }
}

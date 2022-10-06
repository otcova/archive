use super::{ApiState, UtcDate};

#[tauri::command]
pub async fn done_commands_count_vs_days(
    state: tauri::State<'_, ApiState>,
    from: UtcDate,
) -> Result<Option<Vec<usize>>, ()> {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        Ok(Some(database.done_commands_count_vs_days(from)))
    } else {
        Ok(None)
    }
}

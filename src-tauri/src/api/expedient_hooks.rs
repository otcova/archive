use super::ApiState;
use crate::{chunked_database::Uid, expedient_database::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsCallback(usize);

impl JsCallback {
    fn call<T: Serialize>(&self, window: &tauri::Window, argument: &T) {
        let parsed_arg = serde_json::to_string(argument).unwrap();
        let formated_arg = parsed_arg; //.replace("`", "\\`").replace("\\", "\\\\");
        let script = &format!("window.callHook({}, {})", self.0, formated_arg);
        window.eval(script).unwrap();
    }
}

#[tauri::command]
pub fn hook_expedient(
    state: tauri::State<ApiState>,
    window: tauri::Window,
    id: Uid,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(database.as_mut()?.hook_expedient(id, move |expedient| {
        js_callback.call(&window, &expedient);
    }))
}

#[tauri::command]
pub fn hook_all_expedients(
    state: tauri::State<ApiState>,
    window: tauri::Window,
    from: UtcDate,
    limit: usize,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(
        database
            .as_mut()?
            .hook_all_expedients(from, limit, move |expedients| {
                js_callback.call(&window, &expedients)
            }),
    )
}

#[tauri::command]
pub fn hook_all_open_expedients(
    state: tauri::State<ApiState>,
    window: tauri::Window,
    from: UtcDate,
    limit: usize,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(
        database
            .as_mut()?
            .hook_all_open_expedients(from, limit, move |expedients| {
                js_callback.call(&window, &expedients)
            }),
    )
}

#[tauri::command]
pub fn hook_expedient_filter(
    state: tauri::State<ApiState>,
    window: tauri::Window,
    filter: Expedient,
    from: UtcDate,
    limit: usize,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(
        database
            .as_mut()?
            .hook_expedient_filter(filter, from, limit, move |expedients| {
                js_callback.call(&window, &expedients)
            }),
    )
}

#[tauri::command]
pub fn release_hook(state: tauri::State<ApiState>, hook_id: HookId) {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.release_hook(hook_id);
    }
}

#[tauri::command]
pub fn release_all_hooks(state: tauri::State<ApiState>) {
    if let Some(database) = state.database_mutex.lock().unwrap().as_mut() {
        database.release_all_hooks();
    }
}

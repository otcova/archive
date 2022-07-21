use super::*;
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
    expedient_id: Uid,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(database.as_mut()?.hook_expedient(expedient_id, move |expedient| {
        js_callback.call(&window, &expedient);
    }))
}

#[tauri::command]
pub fn hook_list_expedients(
    state: tauri::State<ApiState>,
    window: tauri::Window,
    options: ListExpedientsHookOptions,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(
        database
            .as_mut()?
            .hook_list_expedients(options, move |expedients| {
                js_callback.call(&window, &expedients)
            }),
    )
}

#[tauri::command]
pub fn hook_list_oreders(
    state: tauri::State<ApiState>,
    window: tauri::Window,
    options: ListOrdersHookOptions,
    js_callback: JsCallback,
) -> Option<HookId> {
    let mut database = state.database_mutex.lock().unwrap();
    Some(
        database
            .as_mut()?
            .hook_list_oreders(options, move |expedients| {
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

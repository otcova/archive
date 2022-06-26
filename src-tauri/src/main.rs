#![allow(dead_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(test)]
mod test_utils;

mod api;
mod chunked_database;
mod collections;
mod database;
mod error;
mod expedient_database;
mod mean;

fn main() {
    tauri::Builder::default()
        .manage(api::ApiState::default())
        .invoke_handler(tauri::generate_handler![
            // database state
            api::open_database,
            api::create_database,
            api::rollback_database,
            api::database_rollback_info,
            api::store_database,
            // hooks
            api::hook_expedient,
            api::hook_all_expedients,
            api::hook_all_open_expedients,
            api::hook_expedient_filter,
            api::release_hook,
            api::release_all_hooks,
            // expedients
            api::create_expedient,
            api::update_expedient,
            api::delete_expedient,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

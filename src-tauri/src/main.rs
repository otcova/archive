#![feature(
    thread_spawn_unchecked,
    thread_is_running,
    bool_to_option,
    iter_intersperse
)]
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
mod observable;

use tauri::RunEvent;

fn main() {
    let database_state = api::ApiState::default();

    tauri::Builder::default()
        .manage(database_state.clone())
        .invoke_handler(tauri::generate_handler![
            // database state
            api::open_database,
            api::create_database,
            api::rollback_database,
            api::database_rollback_info,
            api::store_database,
            // hooks
            api::hook_expedient,
            api::hook_list_expedients,
            api::hook_list_orders,
            api::release_hook,
            api::release_all_hooks,
            // expedients
            api::create_expedient,
            api::update_expedient,
            api::delete_expedient,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |_, e| {
            if let RunEvent::ExitRequested { .. } = e {
                if let Some(database) = database_state.database_mutex.lock().unwrap().as_mut() {
                    database.save().expect("error saving database");
                }
            }
        });
}

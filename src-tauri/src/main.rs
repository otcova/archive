#![feature(thread_spawn_unchecked)]
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
            api::hook_list_users,
            api::hook_list_models,
            api::hook_list_license_plates,
            api::hook_list_order_titles,
            api::hook_list_vins,
            api::release_hook,
            api::release_all_hooks,
            // expedients
            api::create_expedient,
            api::update_expedient,
            api::delete_expedient,
            api::read_expedient,
            api::count_expedients,
            api::count_orders,
            // statistics
            api::done_commands_count_vs_days,
            //utils
            utils::download_previous_version,
            utils::install_archive_msi,
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

mod utils {
    use std::fs::{File, remove_file};
    use std::io::{copy, Cursor};
    use std::path::Path;
    use std::process::Command;

    #[tauri::command]
    pub async fn download_previous_version(download_path: &str) -> Result<(), ()> {
        let target = "https://raw.githubusercontent.com/otcova/archive/main/releases/past-msi/archive.msi";
        let response = reqwest::get(target).await.map_err(|_| ())?;

        let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            let fname = Path::new(download_path);
            File::create(fname).map_err(|_| ())
        }?;
        let mut content = Cursor::new(response.bytes().await.map_err(|_| ())?);
        copy(&mut content, &mut dest).map_err(|_| ())?;
        Ok(())
    }

    #[tauri::command]
    pub async fn install_archive_msi(path: &str) -> Result<(), ()> {
        let output = Command::new("cmd")
            .arg("/C")
            .arg(path)
            .arg("/quiet")
            .output()
            .map_err(|_| ())?;
        remove_file(path);
        Ok(())
    }
}

#![feature(thread_spawn_unchecked, drain_filter, string_remove_matches, map_try_insert)]
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
    use std::env::current_exe;
    use std::fs::{remove_file, File};
    use std::io::{copy, Cursor};
    use std::path::Path;
    use std::process::Command;

    #[tauri::command]
    pub async fn download_previous_version(download_path: &str) -> Result<(), ()> {
        let _ = remove_file(download_path);
        let target =
            "https://raw.githubusercontent.com/otcova/archive/main/releases/past-msi/archive.msi";
        let response = reqwest::get(target).await.map_err(|_| ())?;

        let mut dest = {
            response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            File::create(Path::new(download_path)).map_err(|_| ())
        }?;
        let mut content = Cursor::new(response.bytes().await.map_err(|_| ())?);
        copy(&mut content, &mut dest).map_err(|_| ())?;
        Ok(())
    }

    #[tauri::command]
    pub async fn install_archive_msi(path: &str) -> Result<(), ()> {
        let cmd = format!(
            r#"start-process powershell -verb runas -ArgumentList "-C","{}","/quiet",";","timeout -t 1",";","start","'{}'""#,
            path,
            current_exe().map_err(|_| ())?.as_os_str().to_string_lossy()
        );
        let _ = Command::new("powershell").arg("-C").arg(cmd).output();
        Ok(())
    }
}

#![allow(dead_code)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(test)]
mod test_utils;

mod database;
mod expedient_database;

fn main() {
    start_client();
}

fn start_client() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



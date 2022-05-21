#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{generate_handler};
mod image_server;

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![image_server::start_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


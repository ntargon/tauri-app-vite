#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

mod image_server;

fn main() {
  tauri::Builder::default()
    .setup(|app| {

      let app_handle = app.app_handle();
      image_server::start_server(app_handle);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


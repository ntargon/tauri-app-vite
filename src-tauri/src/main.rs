#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod image_server;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {

      image_server::start_server();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


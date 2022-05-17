#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{thread, time::Duration, sync::mpsc};
use tauri::{generate_handler, Manager};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let (tx, rx) = mpsc::channel();

      // server (laser marker <-> tauri app)
      thread::spawn(move || {
        loop {
          tx.send(String::from("hoge")).unwrap();
          println!("send: hoge");
          thread::sleep(Duration::from_secs(1))
        }
      });

      let app_handle = app.app_handle();
      thread::spawn(move || loop {
        let recv = rx.recv().unwrap();
        println!("recv: {}", recv);
        app_handle
            .emit_all("back-to-front", recv)
            .unwrap();
        thread::sleep(Duration::from_secs(1))
      });
      Ok(())
    })
    .invoke_handler(generate_handler![start_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn start_server() -> Result<String, String> {
  for i in 0..10 {
    println!("hey: {}", i);
    // sleep
    thread::sleep(Duration::from_millis(1000));
  }
  // loop {
  //   println!("hey");
  //   // sleep
  //   std::thread::sleep(Duration::from_millis(1000));
  // }
  Ok("Ok".to_string())
}
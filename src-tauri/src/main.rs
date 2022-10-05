#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

let path = "C:/Game Settings"

fn main() {
tauri::Builder::default()
 .run(tauri::generate_context!())
 .expect("error while running tauri application");
}
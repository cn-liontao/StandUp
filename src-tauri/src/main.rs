// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::storage::record::read_storage;
use crate::storage::state::StandingState;

use crate::ui::{init_tray_menu, tray_menu_handler};
use crate::bridge::get_records;

mod ui;
mod utils;
mod storage;
mod bridge;


fn main() {
  let records = read_storage();

  tauri::Builder::default()
    .manage(StandingState::init(records))
    .system_tray(init_tray_menu())
    .on_system_tray_event(|app, event| {
      tray_menu_handler(app, event);
    })
    .invoke_handler(tauri::generate_handler![get_records])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

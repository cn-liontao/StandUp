// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use crate::storage::io::{read_settings, read_storage};
use crate::storage::settings::Settings;
use crate::storage::state::StandingState;

use crate::ui::{init_tray_menu, tray_menu_handler};
use crate::ui::service::stand_or_sit;
use crate::ui::window::create_main_window;

mod ui;
mod utils;
mod storage;
mod bridge;


fn main() {
    let records = read_storage().unwrap_or_else(|err| {
        println!("read_storage failed: {:?}", err);
        vec![]
    });

    let settings = read_settings().unwrap_or_else(|err| {
        println!("read_settings failed: {:?}", err);
        Settings::default()
    });

    let hide_on_start = settings.hide_on_start;

    let app = tauri::Builder::default()
        .setup(move |app| {
            if !hide_on_start {
                create_main_window(app.handle())?;
            }
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                if event.window().label() == "main" {
                    let app = event.window().app_handle();
                    let menu_item = app.tray_handle().get_item("toggle window");
                    menu_item.set_title("打开面板").unwrap();
                }
                api.prevent_close();
            }
            _ => {}
        })
        .manage(StandingState::init(records, settings))
        .system_tray(init_tray_menu(hide_on_start))
        .on_system_tray_event(|app, event| {
            tray_menu_handler(app, event);
        })
        .invoke_handler(tauri::generate_handler![
            bridge::stand_or_sit,
            bridge::get_records,
            bridge::get_settings,
            bridge::save_settings
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            stand_or_sit(app_handle.clone());
        }
        _ => {}
    })
}

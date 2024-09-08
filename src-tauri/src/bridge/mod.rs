use serde_json::Value;
use tauri::{ State, AppHandle, Manager };

use crate::{
    storage::{settings::Settings, state::StandingState},
    ui::service,
};

#[tauri::command]
pub fn stand_or_sit(app_handle: AppHandle) {
    service::stand_or_sit(app_handle);
}

#[tauri::command]
pub fn get_records(state: State<StandingState>) -> Value {
    service::get_records(&state)
}

#[tauri::command]
pub fn get_settings(state: State<StandingState>) -> Value {
    service::get_settings(&state)
}

#[tauri::command]
pub fn save_settings(app_handle: AppHandle, state: State<StandingState>, new_settings: Settings) {
    service::save_settings(&state, new_settings);

    if let Some(main_window) = app_handle.get_window("main") {
        main_window.emit("settings-update", service::get_settings(&state)).unwrap()
    }
}

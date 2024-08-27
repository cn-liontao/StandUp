use serde_json::Value;
use tauri::State;

use crate::{
    storage::{settings::Settings, state::StandingState},
    ui::service,
};

#[tauri::command]
pub fn get_records(state: State<StandingState>) -> Value {
    service::get_records(state)
}

#[tauri::command]
pub fn get_settings(state: State<StandingState>) -> Value {
    service::get_settings(state)
}

#[tauri::command]
pub fn save_settings(state: State<StandingState>, new_settings: Settings) {
    service::save_settings(state, new_settings)
}

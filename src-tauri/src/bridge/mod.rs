use serde_json::Value;
use tauri::State;

use crate::{storage::state::StandingState, ui::service};

#[tauri::command]
pub fn get_records(state: State<StandingState>) -> Value {
    service::get_records(state)
}
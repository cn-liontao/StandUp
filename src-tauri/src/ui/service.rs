use crate::storage::state::StandingState;
use serde_json::Value;
use tauri::State;

pub fn stand_or_sit(state: State<StandingState>) -> bool {
    state.set_standing(!state.is_standing());
    let is_standing = state.is_standing();
    if is_standing {
        stand(state);
    } else {
        sit(state);
    }
    is_standing
}

pub fn get_records(state: State<StandingState>) -> Value {
    state.to_json()
}

fn stand(state: State<StandingState>) -> Value {
    state.append();
    state.flush().unwrap();
    state.to_json()
}

fn sit(state: State<StandingState>) -> Value {
    state.end();
    state.flush().unwrap();
    state.to_json()
}

pub fn get_settings(state: State<StandingState>) -> Value {
    state.settings_json()
}

pub fn save_settings(state: State<StandingState>, new_settings: Settings) {
    state.set_settings(new_settings);
    state.flush_settings().unwrap();
}

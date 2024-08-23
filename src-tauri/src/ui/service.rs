use crate::storage::state::StandingState;
use crate::utils::get_now_timestamp;
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

use serde_json::Value;
use tauri::{AppHandle, Manager, State};
use tokio_util::sync::CancellationToken;
use crate::storage::record::DayRecord;
use crate::storage::settings::Settings;
use crate::storage::state::StandingState;
use crate::ui::notification::schedule_notification;

pub fn stand_or_sit(app_handle: AppHandle) -> bool {
    let state: State<StandingState> = app_handle.state();
    state.set_standing(!state.is_standing());
    let is_standing = state.is_standing();
    if is_standing {
        stand(&state);
    } else {
        sit(&state);
    }

    if state.enable_notification() {
        let cancel_token = CancellationToken::new();
        let cloned_token = cancel_token.clone();
        state.set_notification_task(cancel_token);
        schedule_notification(
            (&app_handle.config()).tauri.bundle.identifier.clone(),
            (if is_standing { "要不要歇会儿？" } else { "已经坐了很久啦" }).to_string(),
            (if is_standing { "已经站了一个小时啦" } else { "站起来活动活动吧" }).to_string(),
            60 * 60,
            cloned_token
        );
    }

    is_standing
}

pub fn get_records(state: &State<StandingState>) -> Value {
    state.to_json()
}

fn stand(state: &State<StandingState>) -> Value {
    state.append();
    state.flush().unwrap();
    state.to_json()
}

fn sit(state: &State<StandingState>) -> Value {
    state.end();
    state.flush().unwrap();
    state.to_json()
}

pub fn merge_records(app: AppHandle, new_records: Vec<DayRecord>) {
    let state = app.state::<StandingState>();
    state.merge(new_records);
    state.flush().unwrap();
}

pub fn get_settings(state: &State<StandingState>) -> Value {
    state.settings_json()
}

pub fn save_settings(state: &State<StandingState>, new_settings: Settings) {
    state.set_settings(new_settings);
    state.flush_settings().unwrap();
}

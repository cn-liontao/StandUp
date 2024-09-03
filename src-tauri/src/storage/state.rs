use serde_json::{json, Value};
use std::sync::Mutex;
use std::borrow::BorrowMut;
use tokio_util::sync::CancellationToken;
use crate::storage::settings::Settings;
use super::record::{merge_records, DayRecord, StandingRecord};
use super::io::{save_setting, save_to_storage};
use crate::utils::{get_now_timestamp, get_today_timestamp};
use crate::utils::errors::{ParsingError, StandingError};

pub struct StandingState {
    standing_now: Mutex<bool>,
    pub day_records: Mutex<Vec<DayRecord>>,
    pub settings: Mutex<Settings>,
    pub settings_status: Mutex<bool>,
    notification_task_token: Mutex<CancellationToken>
}

impl StandingState {
    pub fn init(records: Vec<DayRecord>, settings: Settings) -> Self {
        StandingState {
            standing_now: Mutex::new(false),
            day_records: Mutex::new(records),
            settings: Mutex::new(settings),
            settings_status: Mutex::new(false),
            notification_task_token: Mutex::new(CancellationToken::default())
        }
    }

    pub fn enable_notification(&self) -> bool {
        (*self.settings.lock().unwrap()).enable_notification
    }

    pub fn cancel_notification_task(&self) {
        self.notification_task_token.lock().unwrap().cancel();
    }

    pub fn set_notification_task(&self, cancel_token: CancellationToken) {
        self.cancel_notification_task();
        *self.notification_task_token.lock().unwrap() = cancel_token;
    }

    pub fn is_standing(&self) -> bool {
        *self.standing_now.lock().unwrap()
    }

    pub fn set_standing(&self, is_standing: bool) {
        let mut standing_now = self.standing_now.lock().unwrap();
        *standing_now = is_standing;
    }

    fn map_today<F: Fn(&mut DayRecord) -> ()>(&self, modifier: F) {
        let mut records = self.day_records.lock().unwrap();
        let i = (*records).last_mut();
        let record: &mut DayRecord;
        match i {
            None => {
                (*records).push(DayRecord::default());
                record = (*records).last_mut().unwrap();
            }
            Some(r) => {
                if r.date == get_today_timestamp() {
                    record = r
                } else {
                    (*records).push(DayRecord::default());
                    record = (*records).last_mut().unwrap();
                }
            }
        }

        modifier(record);
    }

    pub fn append(&self) {
        self.map_today(|today_record| {
            today_record.records.push(StandingRecord::default());
        })
    }

    pub fn end(&self) {
        self.map_today(|today_record| {
            if let Some(record) = today_record.records.last_mut() {
                record.end_time = get_now_timestamp();
                record.update_duration();
            }
        });
    }

    // FIXME: support drop specify day's standing reocrd
    pub fn drop(&self, index: usize) -> Result<(), StandingError> {
        let mut records = self.day_records.lock().unwrap();

        if index > records.len() {
            return Err(StandingError {
                message: "No such index in Standing Records.".to_string(),
            });
        }

        (*records).remove(index);
        Ok(())
    }

    pub fn flush(&self) -> Result<(), ParsingError> {
        save_to_storage(&self.day_records.lock().unwrap())
    }

    pub fn to_json(&self) -> Value {
        json!(*self.day_records.lock().unwrap())
    }

    pub fn merge(&self, day_records: Vec<DayRecord>) {
        let mut records = self.day_records.lock().unwrap();

        merge_records(records.borrow_mut(), day_records);
    }

    pub fn set_settings(&self, new_settings: Settings) {
        let mut settings = self.settings.lock().unwrap();

        *settings = new_settings;
    }

    pub fn flush_settings(&self) -> Result<(), ParsingError> {
        save_setting(&self.settings.lock().unwrap())
    }

    pub fn settings_json(&self) -> Value { 
        json!(*self.settings.lock().unwrap()) 
    }
}

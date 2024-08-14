use serde_json::{json, Value};
use std::io;
use std::sync::Mutex;

use crate::storage::record::{save_to_storage, StandingRecord};

use super::record::StandingError;

pub struct StandingState {
    standing_now: Mutex<bool>,
    pub standing_records: Mutex<Vec<StandingRecord>>,
}

impl StandingState {
    pub fn init(records: Vec<StandingRecord>) -> Self {
        StandingState {
            standing_now: Mutex::new(false),
            standing_records: Mutex::new(records),
        }
    }

    pub fn is_standing(&self) -> bool {
        *self.standing_now.lock().unwrap()
    }

    pub fn set_standing(&self, is_standing: bool) {
        let mut standing_now = self.standing_now.lock().unwrap();
        *standing_now = is_standing;
    }

    pub fn append(&self) {
        let mut standing_records = self.standing_records.lock().unwrap();

        (*standing_records).push(StandingRecord::default());
    }

    pub fn drop(&self, index: usize) -> Result<(), StandingError> {
        let mut standing_records = self.standing_records.lock().unwrap();

        if index > standing_records.len() {
            return Err(StandingError {
                message: "No such index in Standing Records.".to_string(),
            });
        }

        (*standing_records).remove(index);
        Ok(())
    }

    pub fn flush(&self) -> io::Result<()> {
        save_to_storage(&self.standing_records.lock().unwrap())
    }

    pub fn to_json(&self) -> Value {
        json!(*self.standing_records.lock().unwrap())
    }
}

use chrono::{Datelike, Local, TimeZone};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub fn get_today_timestamp() -> u128 {
    let now = Local::now();
    let today = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();
    today.timestamp() as u128 * 1000
}

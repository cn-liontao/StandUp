use home::home_dir;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::{fmt, io};

use crate::utils::{get_now_timestamp, get_today_timestamp};

#[derive(Serialize)]
pub struct StandingRecord {
    // Start Timestamp
    pub start_time: u64,
    // End Timestamp
    pub end_time: u64,
    // Stand Duration length (seconds)
    pub duration: u64,
}

impl StandingRecord {
    fn update_duration(&mut self) {
        self.duration = self.end_time - self.start_time
    }
}

impl Default for StandingRecord {
    fn default() -> Self {
        StandingRecord {
            start_time: get_now_timestamp(),
            end_time: 0,
            duration: 0,
        }
    }
}

fn str2time(str_op: Option<&str>) -> Result<u64, ParsingError> {
    match str_op {
        Some(v) => {
            if let Ok(time) = v.to_string().parse::<u64>() {
                Ok(time)
            } else {
                return Err(ParsingError {
                    message: "NaN".to_string(),
                });
            }
        }
        None => {
            return Err(ParsingError {
                message: "String is empty".to_string(),
            });
        }
    }
}

impl TryFrom<String> for StandingRecord {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let mut iter = value.split("|");
        let start = iter.next();
        let end = iter.next();
        let mut record = StandingRecord::default();

        record.start_time = str2time(start)?;
        record.end_time = str2time(end)?;
        record.update_duration();

        Ok(record)
    }
}

impl fmt::Display for StandingRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.start_time, self.end_time)
    }
}

#[derive(Serialize)]
pub struct DayRecord {
    // Date Timestamp (00:00)
    pub date: u64,
    pub records: Vec<StandingRecord>,
}

impl Default for DayRecord {
    fn default() -> Self {
        DayRecord {
            date: get_today_timestamp(),
            records: vec![],
        }
    }
}

fn str2record(record_str: &str) -> Result<StandingRecord, ParsingError> {
    record_str.to_string().try_into()
}

impl TryFrom<String> for DayRecord {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let mut iter = value.split(" ");
        let start = iter.next();
        let end = iter.next();
        let mut day_records = DayRecord::default();

        day_records.date = str2time(start)?;
        if let Some(records_str) = end {
            let records_result: Result<Vec<StandingRecord>, ParsingError> =
                records_str.split(",").map(str2record).collect();
            match records_result {
                Ok(records) => {
                    day_records.records = records;
                }
                Err(err) => return Err(err),
            }
        } else {
            day_records.records = vec![];
        }

        Ok(day_records)
    }
}

impl fmt::Display for DayRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let records_str: Vec<String> = self
            .records
            .iter()
            .map(|record| record.to_string())
            .collect();
        write!(f, "{} {}", self.date, records_str.join(","))
    }
}

pub struct StandingError {
    pub message: String,
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

pub fn read_storage() -> Result<Vec<DayRecord>, ParsingError> {
    let mut local_state_path = home_dir().unwrap();
    local_state_path.push(".local");
    local_state_path.push("state");
    let mut storage_file: Option<File> = None;

    // FIXME: remove solve read_dir().unwrap()
    for x in local_state_path.read_dir().unwrap() {
        match x {
            Ok(file) => {
                if file.file_name().eq(".standing") {
                    storage_file = Some(File::open(file.path().as_path()).unwrap());
                    break;
                }
            }
            Err(err) => {
                return Err(ParsingError {
                    message: "Read dir failed".to_string(),
                })
            }
        }
    }

    let mut records: Vec<DayRecord> = vec![];
    if let Some(file) = storage_file {
        let content = BufReader::new(file);
        for line in content.lines() {
            if let Ok(line_content) = line {
                records.push(line_content.try_into()?);
            }
        }
    }

    Ok(records)
}

fn touch(path: &Path) -> io::Result<File> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}

pub fn save_to_storage(records: &Vec<DayRecord>) -> io::Result<()> {
    let mut local_state_path = home_dir().unwrap();
    local_state_path.push(".local");
    local_state_path.push("state");
    local_state_path.push(".standing");
    let mut storage_file = touch(local_state_path.as_path())?;

    for record in records.iter() {
        writeln!(storage_file, "{}", record.to_string())?;
    }

    Ok(())
}

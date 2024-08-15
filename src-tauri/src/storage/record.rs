use home::home_dir;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::{fmt, io};

use crate::utils::get_now_timestamp;

#[derive(Serialize)]
pub struct StandingRecord {
    // Start Timestamp
    pub start_time: u128,
    // End Timestamp
    pub end_time: u128,
    // Stand Duration length (seconds)
    pub duration: u128,
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

fn str2time(str_op: Option<&str>) -> Result<u128, ParsingError> {
    match str_op {
        Some(v) => {
            if let Ok(time) = v.to_string().parse::<u128>() {
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
pub struct DayRecords {
    // Date Timestamp (00:00)
    date: u128,
    records: Vec<StandingRecord>,
}

impl Default for DayRecords {
    fn default() -> Self {
        DayRecords {
            date: 0,
            records: vec![],
        }
    }
}

fn str2record(record_str: &str) -> Result<StandingRecord, ParsingError> {
    record_str.to_string().try_into()
}

impl TryFrom<String> for DayRecords {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let mut iter = value.split(" ");
        let start = iter.next();
        let end = iter.next();
        let mut day_records = DayRecords::default();

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

impl fmt::Display for DayRecords {
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

pub fn read_storage() -> Result<Vec<StandingRecord>, ParsingError> {
    let mut local_state_path = home_dir().unwrap();
    local_state_path.push(".local");
    local_state_path.push("state");
    let mut storage_file: Option<File> = None;

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

    let mut records: Vec<StandingRecord> = vec![];
    if let Some(file) = storage_file {
        let content = BufReader::new(file);
        for line in content.lines() {
            match line.unwrap().try_into() {
                Ok(record) => {
                    records.push(record);
                }
                Err(parsing_error) => {
                    println!("Parsing storage error: {}", parsing_error.message)
                }
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

pub fn save_to_storage(records: &Vec<StandingRecord>) -> io::Result<()> {
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

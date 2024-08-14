use serde::Serialize;
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::{fmt, io};

use crate::utils::get_now_timestamp;

#[derive(Serialize)]
pub struct StandingRecord {
    pub start_time: u128,
    pub end_time: u128,
}

pub struct StandingError {
    pub message: String,
}

pub struct ParsingError {
    pub message: String,
}

impl Default for StandingRecord {
    fn default() -> Self {
        StandingRecord {
            start_time: get_now_timestamp(),
            end_time: 0,
        }
    }
}

impl TryFrom<String> for StandingRecord {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let mut iter = value.split(" ");
        let start = iter.next();
        let end = iter.next();
        let mut record = StandingRecord::default();

        match start {
            Some(v) => {
                if let Ok(time) = v.to_string().parse::<u128>() {
                    record.start_time = time;
                } else {
                    return Err(ParsingError {
                        message: "NaN".to_string(),
                    });
                }
            }
            None => {
                return Err(ParsingError {
                    message: "Empty line".to_string(),
                });
            }
        }
        match end {
            Some(v) => {
                if let Ok(time) = v.to_string().parse::<u128>() {
                    record.end_time = time;
                } else {
                    return Err(ParsingError {
                        message: "NaN".to_string(),
                    });
                }
            }
            None => {
                return Err(ParsingError {
                    message: "End time not found".to_string(),
                });
            }
        }

        Ok(record)
    }
}

impl fmt::Display for StandingRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.start_time, self.end_time)
    }
}

pub fn read_storage() -> Vec<StandingRecord> {
    let pwd = current_dir().unwrap();
    let mut storage_file: Option<File> = None;

    for x in pwd.read_dir().unwrap() {
        if let Ok(file) = x {
            if file.file_name().eq(".standing") {
                storage_file = Some(File::open(file.path().as_path()).unwrap());
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

    records
}

fn touch(path: &Path) -> io::Result<File> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}

pub fn save_to_storage(records: &Vec<StandingRecord>) -> io::Result<()> {
    let mut storage_file = touch(&Path::new(".standing"))?;

    for record in records.iter() {
        writeln!(storage_file, "{}", record.to_string())?;
    }

    Ok(())
}

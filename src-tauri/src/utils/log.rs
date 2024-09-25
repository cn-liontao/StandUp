use std::io;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use crate::storage::io::with_project_path;
use crate::utils::errors::ParsingError;
use crate::utils::get_today_date;

fn touch(path: &Path) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
}

pub fn init() -> Result<(), ParsingError> {
    with_project_path(|(data_dir, _cfg_dir)| {
        let mut log_file_path = PathBuf::from(data_dir);
        log_file_path.push(format!("standup-{}.log", get_today_date()));
        let log_file = touch(log_file_path.as_path()).map_err(|e| ParsingError::init_str(e.to_string()))?;

        let mut err_file_path = PathBuf::from(data_dir);
        err_file_path.push(format!("standup-{}-err.log", get_today_date()));
        let err_file = touch(err_file_path.as_path()).map_err(|e| ParsingError::init_str(e.to_string()))?;

        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
            WriteLogger::new(LevelFilter::Error, Config::default(), err_file)
        ])?;
        Ok(())
    })
}
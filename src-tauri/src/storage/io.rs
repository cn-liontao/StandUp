use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use crate::storage::record::DayRecord;
use crate::storage::settings::Settings;
use crate::utils::errors::ParsingError;

const STORAGE_FILE_NAME: &str = "record.csv";
const SETTING_FILE_NAME: &str = "settings.ron";

/// Create project path depend on current Os environment
/// return DataLocalDir and ConfigLocalDir from [directories](https://crates.io/crates/directories)
fn with_project_path<F>(cb: F) -> Result<(), ParsingError> where F: FnOnce((&Path, &Path)) -> Result<(), ParsingError> {
    if let Some(project_dir) = ProjectDirs::from("cn", "meowbot", "StandUp") {
        let data_dir = project_dir.data_local_dir();
        let cfg_dir = project_dir.config_local_dir();

        println!("{}", data_dir.to_str().unwrap());

        fs::create_dir_all(data_dir).map_err(|x| ParsingError::init_str(x.to_string()))?;
        fs::create_dir_all(cfg_dir).map_err(|x| ParsingError::init_str(x.to_string()))?;

        cb((data_dir, cfg_dir))
    } else {
        Err(ParsingError { message: "Cannot create project dir".to_string() })
    }
}

pub fn read_storage() -> Result<Vec<DayRecord>, ParsingError> {
    let mut records: Vec<DayRecord> = vec![];
    with_project_path(|(data_dir, _cfg_dir)| {
        let mut storage_file: Option<File> = None;

        for x in data_dir.read_dir().map_err(|e| ParsingError::init("Read dir failed"))? {
            let file = x.map_err(|e| ParsingError {
                message: "Read dir failed".to_string(),
            })?;
            if file.file_name().eq(STORAGE_FILE_NAME) {
                storage_file = Some(File::open(file.path().as_path()).map_err(|e| ParsingError::init("Open CSV failed"))?);
                break;
            }
        }

        if let Some(file) = storage_file {
            let content = BufReader::new(file);
            for line in content.lines() {
                if let Ok(line_content) = line {
                    let record: DayRecord = line_content.try_into()?;
                    if record.date > 0 {
                        records.push(record);
                    }
                }
            }
        }


        Ok(())
    })?;

    Ok(records)
}

fn touch(path: &Path) -> io::Result<File> {
    OpenOptions::new().create(true).write(true).open(path)
}

pub fn save_to_storage(records: &Vec<DayRecord>) -> Result<(), ParsingError> {
    with_project_path(|(data_dir, _cfg_dir)| {
        let mut storage_file_path = PathBuf::from(data_dir);
        storage_file_path.push(STORAGE_FILE_NAME);
        let mut storage_file = touch(storage_file_path.as_path()).map_err(|e| {
            return ParsingError::init_str(e.to_string())
        })?;

        for record in records.iter() {
            writeln!(storage_file, "{}", record.to_string()).map_err(|e| ParsingError::init_str(e.to_string()))?;
        }

        Ok(())
    })
}

pub fn read_settings() -> Result<Settings, ParsingError> {
    let mut settings: Settings = Settings::default();

    with_project_path(|(_data_dir, cfg_dir)| {
        let mut setting_file: Option<File> = None;

        for x in cfg_dir.read_dir().map_err(|e| ParsingError::init("Read config dir failed"))? {
            let file = x.map_err(|e| ParsingError {
                message: "Read config dir failed".to_string(),
            })?;
            if file.file_name().eq(SETTING_FILE_NAME) {
                setting_file = Some(File::open(file.path().as_path()).map_err(|e| ParsingError::init("Open settings failed"))?);
                break;
            }
        }

        if let Some(file) = setting_file {
            let mut content = BufReader::new(file);
            let mut content_str: String = "".to_string();
            content.read_to_string(&mut content_str).unwrap();

            settings = ron::from_str(content_str.as_str()).unwrap();
        }

        Ok(())
    })?;

    Ok(settings)
}

pub fn save_setting(settings: &Settings) -> Result<(), ParsingError> {
    with_project_path(|(_data_dir, cfg_dir)| {
        let mut setting_file_path = PathBuf::from(cfg_dir);
        setting_file_path.push(SETTING_FILE_NAME);

        let mut setting_file = touch(setting_file_path.as_path()).map_err(|e| {
            return ParsingError::init_str(e.to_string())
        })?;

        write!(
            setting_file, "{}",
            ron::ser::to_string_pretty(
                settings,
                ron::ser::PrettyConfig::default()
            ).unwrap()
        ).map_err(|e| ParsingError::init_str(e.to_string()))?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use crate::storage::io::{read_settings, save_setting};
    use crate::storage::settings::Settings;

    #[test]
    fn test_setting_io() {
        let settings = Settings::default();

        save_setting(&settings).unwrap();
        let settings_read = read_settings().unwrap();

        debug_assert_eq!(settings_read.theme, settings.theme);
        debug_assert_eq!(settings_read.start_with_system, settings.start_with_system);
        debug_assert_eq!(settings_read.hide_on_start, settings.hide_on_start);
    }
}
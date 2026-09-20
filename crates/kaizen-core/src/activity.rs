use std::io;
use std::path::{Path};
use chrono::{NaiveDate};
use std::time::Duration;
use thiserror::Error;
use crate::day_log::DayLog;

#[derive(Debug)]
pub enum LogResult {
    Created
}

#[derive(Error, Debug)]
pub enum KaizenError {
    #[error("Invalid base directory")]
    InvalidBaseDirectory,

    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("Parse error")]
    SerdeError(#[from] serde_json::Error),
}

pub fn log_activity(base_directory: &Path, activity_name: &str, date: NaiveDate, duration: Duration) -> Result<LogResult, KaizenError> {
    if !base_directory.is_dir() {
        return Err(KaizenError::InvalidBaseDirectory);
    }

    let activity_directory = base_directory
        .join(activity_name);

    if !activity_directory.exists() {
        std::fs::create_dir_all(&activity_directory)?;
    }

    let day_log_file_name = date.format("%Y-%m-%d").to_string() + ".json";
    let activity_file = activity_directory.join(day_log_file_name);

    let mut day_log = DayLog::new(duration);

    if activity_file.exists() {
        let serialized_activity = std::fs::read_to_string(&activity_file)?;
        day_log = serde_json::from_str(&serialized_activity)?;

        let new_duration = duration + day_log.activity_duration();
        day_log = DayLog::new(new_duration);
    } else {
        std::fs::File::create(&activity_file)?;
    }

    let serialized_log = serde_json::to_string(&day_log)?;
    std::fs::write(activity_file, serialized_log)?;

    Ok(LogResult::Created)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;
    use tempfile::tempdir;
    use crate::day_log::DayLog;
    use super::*;

    #[test]
    fn log_activity_creates_activity_folder() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 9, 20).unwrap();
        let activity_name = "test";

        // Act
        let result = log_activity(temp_dir.path(), activity_name, date, Duration::from_mins(5));

        // Assert
        assert!(result.is_ok());
        assert!(temp_dir.path().join(activity_name).exists());
    }

    #[test]
    fn log_activity_creates_activity_day_log() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 9, 20).unwrap();
        let activity_name = "test";

        // Act
        let result = log_activity(temp_dir.path(), activity_name, date, Duration::from_mins(5));

        // Assert
        assert!(result.is_ok());
        assert!(temp_dir.path().join(activity_name).join("2026-09-20.json").exists());
    }

    #[test]
    fn log_activity_invalid_base_directory() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("not_a_directory.txt");
        std::fs::write(&file_path, "some content").unwrap();

        let date = NaiveDate::from_ymd_opt(2026, 9, 20).unwrap();

        // Act
        let result = log_activity(&file_path, "test", date, Duration::from_mins(5));

        // Assert
        assert!(result.is_err());
        assert_matches!(result, Err(KaizenError::InvalidBaseDirectory));
    }

    #[test]
    fn log_activity_logs_activity() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 9, 20).unwrap();
        let activity_name = "test";
        let duration = Duration::from_mins(5);

        // Act
        let result = log_activity(temp_dir.path(), activity_name, date, duration);

        // Assert
        assert!(result.is_ok());

        let log_file = temp_dir.path().join(activity_name).join("2026-09-20.json");
        let log = std::fs::read_to_string(&log_file).unwrap();

        let day_log: DayLog = serde_json::from_str(&log).unwrap();
        assert_eq!(day_log.activity_duration(), duration);
    }

    #[test]
    fn log_activity_appends_duration() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 9, 20).unwrap();
        let activity_name = "test";
        let duration = Duration::from_mins(5);
        let log_file_path = temp_dir.path().join(activity_name).join("2026-09-20.json");

        std::fs::create_dir_all(log_file_path.parent().unwrap()).unwrap();

        let day_log = DayLog::new(duration);
        let serialized_log = serde_json::to_string(&day_log).unwrap();
        std::fs::write(&log_file_path, serialized_log).unwrap();

        // Act
        let result = log_activity(temp_dir.path(), activity_name, date, duration);

        // Assert
        assert!(result.is_ok());

        let log = std::fs::read_to_string(&log_file_path).unwrap();

        let day_log: DayLog = serde_json::from_str(&log).unwrap();
        assert_eq!(day_log.activity_duration(), Duration::from_mins(10));
    }
}
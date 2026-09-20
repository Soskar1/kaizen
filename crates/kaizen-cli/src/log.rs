use std::env;
use chrono::Utc;
use clap::Args;
use colored::Colorize;
use thiserror::Error;
use kaizen_core::activity::log_activity;
use crate::log::LogError::ActivityError;

#[derive(Args)]
pub struct LogArgs {
    activity_name: String,
    duration_in_seconds: u64
}

#[derive(Error, Debug)]
pub enum LogError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ActivityError(#[from] kaizen_core::activity::ActivityError)
}

pub fn log(args: LogArgs) -> Result<(), LogError> {
    let directory = env::current_dir()?;
    let date = Utc::now().date_naive();

    let result = log_activity(&directory, &args.activity_name, date, args.duration_in_seconds);

    if !result.is_ok() {
        return Err(ActivityError(result.unwrap_err()));
    }

    println!("{}", "Activity logged successfully!".green());

    Ok(())
}
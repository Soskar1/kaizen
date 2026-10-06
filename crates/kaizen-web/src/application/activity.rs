use chrono::NaiveDate;

use crate::{domain::activity_log_range::ActivityLogRange, integration::client::{create_activity, get_activity_logs, log_activity_time}};

pub async fn collect_logs(activity_name: &str, from: NaiveDate, to: NaiveDate) -> Option<ActivityLogRange> {
    match get_activity_logs(&activity_name, from, to).await {
        Ok(logs) => {
            Some(ActivityLogRange::new(from, to, logs))
        }
        Err(_) => {
            // TODO: add logging
            None
        }
    }
}

pub async fn try_create_activity(activity_name: &str) -> Result<(), ()> {
    if let Err(_) = create_activity(&activity_name).await {
        // TODO: add logging
        return Err(())
    }

    Ok(())
}

pub async fn try_log_activity_time(activity_name: &str, activity_duration_in_seconds: u64, date: NaiveDate) -> Result<(), ()> {
    if let Err(_) = log_activity_time(&activity_name, activity_duration_in_seconds, date).await {
        // TODO: add logging
        return Err(())
    }

    Ok(())
}
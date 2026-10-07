use std::collections::HashMap;

use chrono::NaiveDate;
use gloo_net::{
    http::Request,
    Error as GlooNetError,
};
use serde::{Deserialize, Serialize};
use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    GlooNetError(#[from] GlooNetError),

    #[error("Server returned status {0}")]
    FailedRequest(u16)
}

const BASE_ADDRESS: &'static str = "http://127.0.0.1:3000";

fn get_activities_address() -> String {
    format!("{}/activities", BASE_ADDRESS)
}

fn get_logs_address(activity_name: &str) -> String {
    format!("{}/activities/{}/logs", BASE_ADDRESS, activity_name)
}

pub async fn get_activities() -> Result<Vec<String>, ServerError> {
    let activities_address = get_activities_address();
    let response = Request::get(&activities_address)
        .send()
        .await?;

    if !response.ok() {
        return Err(ServerError::FailedRequest(response.status()));
    }

    let activity_names = response
        .json::<Vec<String>>()
        .await?;

    Ok(activity_names)
}

pub async fn create_activity(activity_name: &str) -> Result<(), ServerError> {
    let activities_address = get_activities_address();
    let response = Request::post(&activities_address)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(activity_name)?
        .send()
        .await?;

    if !response.ok() {
        return Err(ServerError::FailedRequest(response.status()));
    }

    Ok(())
}

pub async fn log_activity_time(activity_name: &str, activity_duration_in_seconds: u64, date: NaiveDate) -> Result<(), ServerError> {
    let activity_log: ActivityLog = ActivityLog {
        activity_duration_in_seconds,
        date
    };

    let logs_address = get_logs_address(activity_name);
    let response = Request::post(&logs_address)
        .json(&activity_log)?
        .send()
        .await?;

    if !response.ok() {
        return Err(ServerError::FailedRequest(response.status()))
    }
    
    Ok(())
}

pub async fn get_activity_logs(activity_name: &str, from: NaiveDate, to: NaiveDate) -> Result<HashMap<NaiveDate, u64>, ServerError> {
    let logs_address = get_logs_address(activity_name);

    let response = Request::get(&logs_address)
        .query([("from", from.to_string())])
        .query([("to", to.to_string())])
        .send()
        .await?;

    if !response.ok() {
        return Err(ServerError::FailedRequest(response.status()))
    }

    let date_to_duration: HashMap<NaiveDate, u64> = response
        .json::<Vec<ActivityLog>>()
        .await?
        .iter()
        .map(|activity_log| (activity_log.date, activity_log.activity_duration_in_seconds))
        .collect();

    Ok(date_to_duration)
}

#[derive(Serialize, Deserialize)]
struct ActivityLog {
    activity_duration_in_seconds: u64,
    date: NaiveDate
}
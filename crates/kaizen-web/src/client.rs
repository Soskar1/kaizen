use chrono::NaiveDate;
use gloo_net::{
    http::Request,
    Error as GlooNetError,
};
use serde::Serialize;
use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    GlooNetError(#[from] GlooNetError),

    #[error("Server returned status {0}")]
    FailedRequest(u16)
}

const BASE_ADDRESS: &'static str = "http://127.0.0.1:3000";

fn activities_address() -> String {
    format!("{}/activities", BASE_ADDRESS)
}

fn time_address() -> String {
    format!("{}/time", BASE_ADDRESS)
}

pub async fn get_activities() -> Result<Vec<String>, ServerError> {
    let activities_address = activities_address();
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
    let activities_address = activities_address();
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
        activity_name: activity_name.to_string(),
        activity_duration_in_seconds,
        date
    };

    let time_address = time_address();
    let response = Request::post(&time_address)
        .json(&activity_log)?
        .send()
        .await?;

    if !response.ok() {
        return Err(ServerError::FailedRequest(response.status()))
    }
    
    Ok(())
}


#[derive(Serialize)]
struct ActivityLog {
    activity_name: String,
    activity_duration_in_seconds: u64,
    date: NaiveDate
}
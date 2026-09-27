use gloo_net::{Error::{self}, http::Request};
use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    GlooNetError(#[from] Error),

    #[error("Server returned status {0}")]
    FailedRequest(u16)
}

const BASE_ADDRESS: &'static str = "http://127.0.0.1:3000";

fn activities_address() -> String {
    format!("{}/activities", BASE_ADDRESS)
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
        .header("Content-Type", "text/plain; charset=utf8")
        .body(activity_name)?
        .send()
        .await?;

    if !response.ok() {
        return Err(ServerError::FailedRequest(response.status()));
    }

    Ok(())
}
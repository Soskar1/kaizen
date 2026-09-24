use gloo_net::http::Request;

const BASE_ADDRESS: &'static str = "http://127.0.0.1:3000";

fn activities_address() -> String {
    format!("{}/activities", BASE_ADDRESS)
}

pub async fn get_activities() -> Result<Vec<String>, String>{
    let activities_address = activities_address();
    let response = Request::get(&activities_address)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("Server returned status {}", response.status()));
    }

    let activity_names = response
        .json::<Vec<String>>()
        .await
        .map_err(|error| error.to_string())?;

    Ok(activity_names)
}
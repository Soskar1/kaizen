use std::{env, fs, io};
use std::path::Path;
use axum::{routing::get, Router, Json};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::http::header::CONTENT_TYPE;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://[::1]:8080".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/activities", get(get_activities).post(post_activities))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_activities() -> Result<Json<Vec<String>>, StatusCode> {
    let path = env::current_dir().map_err(|error| {
        eprintln!("Failed to get current directory: {:?}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let activities = read_activity_names(&path).map_err(|error|{
        eprintln!("Failed to read activities: {:?}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(activities))
}

fn read_activity_names(path: &Path) -> io::Result<Vec<String>> {
    let paths = fs::read_dir(path)?;

    let mut activities: Vec<String> = vec!();

    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            activities.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    Ok(activities)
}

async fn post_activities(body: String) -> StatusCode {
    let name = body.trim();

    println!("Received activity name: {name}");

    fs::create_dir(format!("C:\\Projects\\Rust\\{name}")).unwrap();

    StatusCode::CREATED
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tempfile::tempdir;
    use super::*;

    #[rstest]
    #[case(vec!("Rust", "Learning"))]
    #[case(vec!())]
    fn read_activity_names_must_return_all_activity_names(#[case] directories_to_create: Vec<&str>) {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        for directory_name in directories_to_create.clone() {
            let directory_to_create = temp_dir_path.join(directory_name);
            fs::create_dir(directory_to_create).unwrap();
        }

        // Act
        let activity_names = read_activity_names(temp_dir_path).unwrap();

        // Assert
        let expected = directories_to_create
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        for name in activity_names.clone() {
            assert!(expected.contains(&name));
        }

        for expected_name in expected {
            assert!(activity_names.contains(&expected_name));
        }
    }
}
use std::{env, fs, io};
use std::path::{Component, Path, PathBuf};
use axum::extract::State;
use axum::{routing::get, Router, Json};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::http::header::CONTENT_TYPE;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    activities_directory: PathBuf
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let activities_directory = env::current_dir()?;

    let state = AppState {
        activities_directory,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://[::1]:8080".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/activities", get(get_activities).post(post_activities))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}

async fn get_activities(State(state): State<AppState>) -> Result<Json<Vec<String>>, StatusCode> {
    let activities = read_activity_names(&state.activities_directory).map_err(|error|{
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

async fn post_activities(State(state): State<AppState>, body: String) -> Result<StatusCode, StatusCode> {
    let name = body.trim();

    println!("Received activity name: {name}");

    create_activity_folder(&state.activities_directory, name).map_err(|error| {
        match error.kind() {
            io::ErrorKind::InvalidInput => StatusCode::BAD_REQUEST,
            io::ErrorKind::AlreadyExists => StatusCode::CONFLICT,
            _ => {
                eprintln!("Failed to create an activity directory: {error:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    })?;

    Ok(StatusCode::CREATED)
}

fn create_activity_folder(activities_directory: &Path, activity_name: &str) -> io::Result<()> {
    if activity_name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Activity name is empty"
        ));
    }

    debug_assert!(!activity_name.starts_with(" "));
    debug_assert!(!activity_name.ends_with(" "));
    
    let mut components = Path::new(activity_name).components();
    let first_part = components.next();
    let second_part = components.next();
    match (first_part, second_part) {
        (Some(Component::Normal(_)), None) => {}
        _ => {
            return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Activity name must be a single directory name"
        ));
        }
    }

    let new_activity_path = activities_directory.join(activity_name);

    fs::create_dir(new_activity_path)
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

    #[rstest]
    #[case("TestActivity")]
    #[case("Test Activity")]
    fn create_activity_folder_creates_folder(#[case] activity_name: &str) {
        // Arrange
        let temp_dir: tempfile::TempDir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        
        // Act
        let result = create_activity_folder(temp_dir_path, activity_name);

        // Assert
        assert!(result.is_ok());

        let new_activity_directory_path = temp_dir_path.join(activity_name);
        assert!(new_activity_directory_path.is_dir())
    }

    #[test]
    fn create_activity_folder_does_not_allow_duplicates() {
        // Arrange
        let temp_dir: tempfile::TempDir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        let test_activity = "TestActivity";

        let test_activity_path = temp_dir_path.join(test_activity);
        fs::create_dir(test_activity_path).unwrap();

        // Act
        let result = create_activity_folder(temp_dir_path, test_activity);

        // Assert
        assert!(result.is_err())
    }

    #[rstest]
    #[case("")]
    #[case("/")]
    #[case("\\")]
    #[case("<")]
    #[case(">")]
    #[case(":")]
    #[case("\"")]
    #[case("|")]
    #[case("?")]
    #[case("*")]
    #[case("asb<sdf")]
    #[case("asb?sdf")]
    #[case("hello\\world")]
    #[case("./hello/world")]
    #[case("C:\\something")]
    #[case("..\\Outside")]
    #[case("..")]
    #[case(".")]
    fn create_activity_folder_prohibits_invalid_activity_names(#[case] activity_name: &str) {
        // Arrange
        let temp_dir: tempfile::TempDir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        // Act
        let result = create_activity_folder(temp_dir_path, activity_name);

        // Assert
        assert!(result.is_err())
    }
}
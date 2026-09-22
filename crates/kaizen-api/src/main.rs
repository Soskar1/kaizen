use std::{env, fs};
use axum::{
    routing::get,
    Router,
};
use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/activities", get(get_activities).post(post_activities));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_activities() {

}

async fn post_activities(body: String) -> StatusCode {
    let name = body.trim();

    println!("Received activity name: {name}");

    fs::create_dir(format!("C:\\Projects\\Rust\\{name}")).unwrap();

    StatusCode::CREATED
}
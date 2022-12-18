use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root))
        .route("/sleep/:duration", get(sleep));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("Server Listening on {:?}",address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();    
}

async fn root() -> &'static str {
    "/sleep/:duration"
}

#[derive(serde::Serialize)]
struct Sleep {
    start: SystemTime,
    end: SystemTime,
    duration: u64,
    message: &'static str,
}

async fn sleep(Path(duration): Path<u64>) -> impl IntoResponse {
    let start = SystemTime::now();
    tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
    let end = SystemTime::now();
    (StatusCode::OK,
        Json(Sleep {
            start,
            end,
            duration,
            message: "Success"})
    )
}
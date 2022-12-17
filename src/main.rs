use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(root))
    .route("/sleep/:duration", get(sleep));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "/sleep/:time"
}

#[derive(serde::Serialize)]
struct Sleep {
    start: SystemTime,
    end: SystemTime,
    duration: u64,
    message: String,
}

async fn sleep(Path(duration): Path<u64>) -> impl IntoResponse {
    let start = SystemTime::now();
    tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
    let end = SystemTime::now();
    (
        StatusCode::OK,
        Json(Sleep {
            duration,
            message: "Success".to_owned(),
            start,
            end,
        }),
    )
}

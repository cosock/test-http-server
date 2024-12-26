use axum::{extract::Path, Router, routing::get};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    let app = Router::new()
        .route("/delay/:secs", get(delay));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

async fn delay(secs: Path<u64>) {
    tokio::time::sleep(std::time::Duration::from_secs(*secs)).await;
}

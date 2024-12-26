use std::{net::SocketAddr, path::PathBuf};

use axum::{extract::Path, Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    let mut args = std::env::args().skip(1);
    let port = args.next().and_then(|s| {
        s.parse::<u16>().ok()
    }).unwrap_or(8080);
    let cert_dir = args.next().map(PathBuf::from);

    let app = Router::new()
        .route("/delay/:secs", get(delay));
    let Some(cert_dir) = cert_dir else {
        return run_http_server(app, port).await
    };
    run_https_server(app, port, cert_dir).await
}

async fn run_http_server(app: Router, port: u16) -> Result {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

async fn run_https_server(app: Router, port: u16, cert_dir: PathBuf) -> Result {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let config = RustlsConfig::from_pem_file(
        cert_dir
            .join("cert.pem"),
        cert_dir
            .join("key.pem"),
    )
    .await
    ?;
    Ok(axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?)
}

async fn delay(secs: Path<u64>) {
    tokio::time::sleep(std::time::Duration::from_secs(*secs)).await;
}

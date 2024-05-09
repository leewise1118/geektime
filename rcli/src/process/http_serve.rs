use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    // axum router
    let state = HttpServeState { path: path.clone() };

    let dir_services = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    let router = Router::new()
        .nest_service("/tower", dir_services)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        (StatusCode::NOT_FOUND, "file not found".to_string())
    } else {
        if p.is_dir() {
            let mut html = String::from("<html><body><ul>");
            p.read_dir().unwrap().for_each(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let name = path.file_name().unwrap().to_string_lossy();
                    let mut dash = String::from("");
                    if entry.path().is_dir() {
                        dash = String::from("/");
                    }
                    html.push_str(&format!(
                        "<li><a href=\"{}{}\">{}{}</a></li>",
                        name, dash, name, dash
                    ))
                }
            });
            html.push_str("</ul></body></html>");
            (StatusCode::OK, html)
        } else {
            match tokio::fs::read_to_string(p).await {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, content)
                }
                Err(e) => {
                    warn!("Error reading file: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                }
            }
        }
    }
}

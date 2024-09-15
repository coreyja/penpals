use std::path::PathBuf;

use axum::{
    http::{HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root)).fallback(assets);

    for entry in FRONTEND_DIST_DIR.entries() {
        println!("entry: {}", entry.path().to_string_lossy());
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use include_dir::{include_dir, Dir};

static FRONTEND_DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist");

async fn root() -> impl IntoResponse {
    let path = PathBuf::from("index.html");
    serve_file(&path)
}

async fn assets(uri: Uri) -> impl IntoResponse {
    let path = PathBuf::from(uri.path());
    let filename = path.file_name().unwrap().to_str().unwrap();
    let path = PathBuf::from(filename);

    serve_file(&path)
}

fn serve_file(path: &PathBuf) -> impl IntoResponse {
    let file = FRONTEND_DIST_DIR.get_file(path);

    if let Some(file) = file {
        let mime = mime_guess::from_path(path)
            .first_raw()
            .map(HeaderValue::from_static)
            .unwrap_or_else(|| {
                HeaderValue::from_str(mime::APPLICATION_OCTET_STREAM.as_ref()).unwrap()
            });
        let body = axum::body::Body::from(file.contents());
        Response::builder()
            .header("Content-Type", mime)
            .body(body)
            .unwrap()
    } else {
        (StatusCode::NOT_FOUND, "Not Found").into_response()
    }
}

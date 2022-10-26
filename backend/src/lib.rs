mod alias;
mod types;

pub use crate::alias::alias_replacement;
pub use crate::types::{RequestAlias, ResponseMatches};

// Web framework
use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

// File IO
use std::fs::read_to_string;
use std::path::PathBuf;

pub async fn start_web_server() {
    let app = Router::new()
        .route("/api", post(get_matching_aliases))
        .route("/echo", get(echo))
        // .route("/api", get(|| async {"Hello"}))
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_origin(Any)
                .allow_headers(Any),
        );

    // let addr = "0.0.0.0:3000";
    let addr = "127.0.0.1:3000";

    println!("Serving on {addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_matching_aliases(
    extract::Json(payload): extract::Json<RequestAlias>,
) -> Json<ResponseMatches> {
    // dbg!(&payload);
    dbg!(alias_replacement(payload.clone()));
    Json(alias_replacement(payload))
}

async fn echo(extract::Json(payload): extract::Json<RequestAlias>) -> Json<RequestAlias> {
    Json(payload)
}

pub fn run_on_file(path: PathBuf, target_type: String) -> ResponseMatches {
    let source = read_to_string(path).unwrap();
    let payload = RequestAlias {
        source,
        target_type,
    };

    alias_replacement(payload)
}

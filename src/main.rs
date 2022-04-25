mod alias;

use crate::alias::{alias_replacement, RequestAlias, ResponseMatches};

// // CLI library
// use clap::Parser as CLIParser;
// use std::fs::read_to_string;

// Web framework
use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

// #[derive(CLIParser, Debug)]
// #[clap(author, version, about, long_about = None)]
// pub struct Args {
//     type_sig: String,

//     #[clap(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

#[tokio::main]
async fn main() {
    // let args = Args::parse();

    // let source_code = read_to_string(args.path).unwrap();
    // let source = source_code.as_bytes();
    //
    // let json = alias_replacement(&args.type_sig, source);
    // println!("{}", json.to_string());

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

async fn get_matching_aliases(
    extract::Json(payload): extract::Json<RequestAlias>,
) -> Json<ResponseMatches> {
    // dbg!(&payload);
    dbg!(alias_replacement(payload.clone()));
    Json(alias_replacement(payload))
}

async fn echo(extract::Json(payload): extract::Json<RequestAlias>) -> Json<RequestAlias> {
    Json(payload)
}

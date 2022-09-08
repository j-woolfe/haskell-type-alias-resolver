mod alias;

use crate::alias::{alias_replacement, RequestAlias, ResponseMatches};

// CLI library
use clap::Parser as CLIParser;
use std::fs::read_to_string;
use std::path::PathBuf;

// Web framework
use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

use serde_json;

#[derive(CLIParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Runs as a web server on port 3000
    #[clap(short, long, action)]
    server: bool,

    /// Path of Haskell file to analyse
    #[clap(short, long, value_parser, value_name = "FILE")]
    path: Option<PathBuf>,

    /// Target type alias
    #[clap(short, long, value_parser, value_name = "TYPE")]
    target: Option<String>,

    /// Enable human readable output
    #[clap(short = 'r', long, action)]
    human_readable: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.server {
        start_web_server().await;
    } else {
        // Run in command line mode
        // TODO:: Add error checking for missing/invalid path

        if let (Some(path), Some(target_type)) = (args.path, args.target) {
            // let source = read_to_string(path).unwrap();
            let source = read_to_string(path).unwrap();
            let payload = RequestAlias {
                source,
                target_type,
            };

            let output = alias_replacement(payload);

            if args.human_readable {
                println!("{}", output);
            } else {
                println!("{}", serde_json::to_string(&output).unwrap());
            }
        } else {
            //TODO: Improve errors
            println!("Missing arguments")
        }
    }
}

async fn start_web_server() {
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

mod v0;

use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use axum::Router;
use tower_http::trace::TraceLayer;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // let s = fs::read_to_string("d20.toml").unwrap();
    // let server: Server = toml::from_str(&s).unwrap();

    // // TODO: support config
    // // TODO: websockets
    // // TODO: feature flags for multi-tenant deployment
    // let app = Router::new()
    //     .nest("/v0", v0::router())
    //     .layer(TraceLayer::new_for_http());
    //
    // axum::Server::bind(&"0.0.0.0:42002".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
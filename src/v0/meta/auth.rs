use anyhow::Error;
use async_trait::async_trait;
use axum::extract::{Extension, TypedHeader};
use axum::headers::authorization::Basic;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::{
    extract::{extractor_middleware, FromRequest, RequestParts},
    http,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::join;
use tokio::sync::Mutex;

// TODO: should authentication even be here? doesn't this type belong in core?
pub enum Authentication {
    Basic { username: String, password: String },
    // TODO: add bearer tokens
}

#[async_trait]
impl<B> FromRequest<B> for Authentication
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Err(StatusCode::UNAUTHORIZED)
    }
}

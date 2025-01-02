use std::sync::Arc;

use aksono_common::app::App;
use axum::{
    extract::{Request, State},
    http,
    middleware::{self, Next},
    response::Response,
    routing::get,
    RequestExt as _, Router,
};
use bytes::{BufMut as _, BytesMut};
use http_body_util::{BodyExt as _, Collected};
use ruma::{api::client::error::ErrorKind, CanonicalJsonValue};

use crate::error::Error;

async fn my_middleware(State(app): State<Arc<App>>, request: Request, next: Next) -> Response {
}

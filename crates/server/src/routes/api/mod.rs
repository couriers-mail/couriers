pub mod v1;

use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/", get(root_handler))
}

async fn root_handler() -> Response {
    ().into_response()
}

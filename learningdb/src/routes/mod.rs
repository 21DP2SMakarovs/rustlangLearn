mod hellowrld;

use axum::{
    body::Body,
    routing::{get},
    Router,
};
use hellowrld::hello_world;

pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/", get(hello_world))

}
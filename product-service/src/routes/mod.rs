use axum::{routing::get, Router};
use commons::setup::app::RouteManager;

pub struct AppRouteManager;

impl AppRouteManager {
    pub fn new() -> Self {
        Self
    }
}

impl RouteManager for AppRouteManager {
    fn setup_routes(&self) -> Router {
        Router::new().route("/", get(|| async { "Hello World" }))
    }
}

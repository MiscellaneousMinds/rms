use axum::Router;

pub trait RouteManager {
    fn setup_routes(&self) -> Router;
}

pub struct Application {
    app: Router,
    host: String,
    port: u16,
}

impl Application {
    pub async fn new<M: RouteManager>(host: &str, port: u16, route_manager: M) -> Self {
        let app = route_manager.setup_routes();

        Self {
            host: host.to_string(),
            port,
            app,
        }
    }

    pub async fn serve(self) -> Result<(), std::io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        tracing::info!("listening on {}", addr);
        axum::Server::bind(&addr.parse().unwrap())
            .serve(self.app.into_make_service())
            .await
            .unwrap();
        Ok(())
    }
}

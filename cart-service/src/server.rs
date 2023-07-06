use crate::{config::get_config, routes::AppRouteManager};
use commons::setup::app::Application;

pub struct Server {
    app: Application,
}

impl Server {
    pub async fn new() -> Self {
        // load config
        let config = get_config().expect("Failed to get config.");

        // Setup logging
        // init_subscriber(config.log.format.as_str(), config.log.level.as_str());

        // init app
        let route_manager = AppRouteManager::new();
        let app = Application::new(&config.app.host, config.app.port, route_manager).await;
        Self { app }
    }

    pub async fn start(self) -> Result<(), std::io::Error> {
        self.app.serve().await
    }
}

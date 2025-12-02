pub mod config;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;
pub mod docs;
pub mod utils;

use actix_web::{App, HttpServer, web};
use env_logger;
use log::LevelFilter;
use config::AppConfig;
use routes::prices::get_average;
use docs::swagger::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = AppConfig::from_file("Config.toml").expect("Failed to load config");

    env_logger::Builder::new()
        .filter_level(cfg.logging.level.parse().unwrap_or(LevelFilter::Info))
        .init();

    let bind = format!("{}:{}", cfg.server.bind_address, cfg.server.port);
    println!("Rust API running at http://{}", bind);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cfg.clone()))
            .service(get_average)
            .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(bind)?
    .run()
    .await
}

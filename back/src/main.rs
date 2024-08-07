use std::net::SocketAddr;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;
use log::{info, error};
use env_logger::Env;

use crate::api::api_config;
use crate::minesweeper::resource::{minesweeper_scope};
use crate::property::Config;

pub mod minesweeper;
mod property;
mod api;
pub mod constant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Config::get_properties();
    let port = config.port();
    let address = SocketAddr::from(([0, 0, 0, 0], config.port()));

    info!("Starting server at port:{}", port);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
                               .allow_any_header()
                               .allow_any_method()
                               .allow_any_origin()
                               .max_age(3600);
        App::new().wrap_api_with_spec(api_config())
                  .wrap(cors).service(minesweeper_scope())
                  .with_json_spec_at("/api/spec/v2")
                  .with_json_spec_v3_at("/api/spec/v3")
                  .with_swagger_ui_at("/swagger-ui/")
                  .build()
        }).bind(address);

    match server {
        Ok(srv) => {
            info!("Server running at port:{}", port);
            srv.run().await
        },
        Err(e) => {
            error!("Failed to bind server to port:{} - {}", port, e);
            Err(e)
        }
    }
}
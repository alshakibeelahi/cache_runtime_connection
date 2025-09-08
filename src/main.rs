mod cluster;
mod handlers;
mod routing;
mod state;

use crate::{cluster::ClusterPools, routing::ServiceRoutingConfig, state::AppState};
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tokio::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let state = web::Data::new(AppState {
        clusters: Arc::new(RwLock::new(ClusterPools::new())),
        routing: Arc::new(RwLock::new(ServiceRoutingConfig::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/connect", web::post().to(handlers::connect_handler))
            .route("/check/{service}", web::get().to(handlers::check_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use crate::{cluster::ClusterPools, routing::ServiceRoutingConfig};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub clusters: Arc<RwLock<ClusterPools>>,
    pub routing: Arc<RwLock<ServiceRoutingConfig>>,
}

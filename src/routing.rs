use std::collections::HashMap;

pub struct ServiceRoutingConfig {
    pub routes: HashMap<String, (String, String)>, // service -> (kafka_id, redis_id)
}

impl ServiceRoutingConfig {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add_mapping(&mut self, service: &str, kafka: &str, redis: &str) {
        self.routes.insert(service.to_string(), (kafka.to_string(), redis.to_string()));
    }

    pub fn get_mapping(&self, service: &str) -> Option<(String, String)> {
        self.routes.get(service).cloned()
    }
}

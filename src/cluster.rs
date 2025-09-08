use std::collections::HashMap;
use deadpool_redis::Pool as RedisPool;
use rdkafka::producer::FutureProducer;

pub struct ClusterPools {
    pub kafka: HashMap<String, FutureProducer>,
    pub redis: HashMap<String, RedisPool>,
}

impl ClusterPools {
    pub fn new() -> Self {
        Self {
            kafka: HashMap::new(),
            redis: HashMap::new(),
        }
    }

    pub fn add_kafka_cluster(&mut self, key: &str, producer: FutureProducer) {
        self.kafka.insert(key.to_string(), producer);
    }

    pub fn add_redis_cluster(&mut self, key: &str, pool: RedisPool) {
        self.redis.insert(key.to_string(), pool);
    }
}

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use deadpool_redis::{Config as RedisConfig, redis::AsyncCommands};

use crate::{state::AppState};

#[derive(Debug, Deserialize)]
pub struct ConnectRequest {
    pub service: String,
    pub kafka_broker: String,
    pub redis_url: String,
}

pub async fn connect_handler(
    state: web::Data<AppState>,
    req: web::Json<ConnectRequest>,
) -> HttpResponse {
    let kafka_key = format!("kafka_{}", req.service);
    let redis_key = format!("redis_{}", req.service);

    // Create Kafka producer
    let producer = match ClientConfig::new()
        .set("bootstrap.servers", &req.kafka_broker)
        .create::<FutureProducer>()
    {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Kafka error: {e}")),
    };

    // Create Redis pool
    let pool = match RedisConfig::from_url(&req.redis_url)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
    {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Redis error: {e}")),
    };

    // Save to state
    {
        let mut clusters = state.clusters.write().await;
        let mut routing = state.routing.write().await;

        clusters.add_kafka_cluster(&kafka_key, producer);
        clusters.add_redis_cluster(&redis_key, pool);
        routing.add_mapping(&req.service, &kafka_key, &redis_key);
    }

    HttpResponse::Ok().body(format!("Service {} registered", req.service))
}

pub async fn check_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let service = path.into_inner();

    // Get the mapping
    let mapping = {
        let routing = state.routing.read().await;
        routing.get_mapping(&service)
    };

    if mapping.is_none() {
        return HttpResponse::NotFound().body("Service not found in routing table");
    }

    let (kafka_id, redis_id) = mapping.unwrap();

    // --------------------- Redis check ---------------------
    let redis_val = {
        let clusters = state.clusters.read().await;
        let pool = match clusters.redis.get(&redis_id) {
            Some(p) => p,
            None => return HttpResponse::InternalServerError().body("Redis pool not found"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => return HttpResponse::InternalServerError()
                .body(format!("Redis pool error: {}", e)),
        };

        // Explicit type annotation for SET
        if let Err(e) = conn.set::<&str, &str, ()>("ping", "pong").await {
            return HttpResponse::InternalServerError()
                .body(format!("Redis SET failed: {}", e));
        }

        match conn.get::<_, String>("ping").await {
            Ok(val) => val,
            Err(e) => return HttpResponse::InternalServerError()
                .body(format!("Redis GET failed: {}", e)),
        }
    };

    // --------------------- Kafka check ---------------------
    let kafka_result = {
        let clusters = state.clusters.read().await;
        let producer: &FutureProducer = match clusters.kafka.get(&kafka_id) {
            Some(p) => p,
            None => return HttpResponse::InternalServerError().body("Kafka producer not found"),
        };

        // Dynamic topic per service
        let topic_name = "notch_broadcast";

        let record = FutureRecord::to(&topic_name)
            .key("check")
            .payload("hello_kafka");

        match producer.send(record, Duration::from_secs(1)).await {
            Ok(_) => "Kafka send OK".to_string(),
            Err((e, _)) => return HttpResponse::InternalServerError()
                .body(format!("Kafka send failed: {}", e)),
        }
    };

    HttpResponse::Ok().body(format!("Redis: {}, Kafka: {}", redis_val, kafka_result))
}

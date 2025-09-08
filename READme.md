# Dynamic Kafka + Redis Runtime Connection

## Project Structure

```
micro-orchestrator/
├── Cargo.toml
└── src/
    ├── main.rs          # Starts server, registers routes
    ├── handlers.rs      # /connect and /check handlers
    ├── state.rs         # AppState with Arc<RwLock<ClusterPools>> and routing
    ├── cluster.rs       # ClusterPools struct with HashMaps for Kafka & Redis
    └── routing.rs       # ServiceRoutingConfig mapping service → (kafka_id, redis_id)
```

---

### File Responsibilities

| File          | Purpose                                                                 |
| ------------- | ----------------------------------------------------------------------- |
| `main.rs`     | Start Actix Web server, define routes `/connect` and `/check/{service}` |
| `handlers.rs` | Handle requests: register new service & check connections               |
| `state.rs`    | Shared state container with Kafka producers & Redis pools               |
| `cluster.rs`  | In-memory pools for Kafka and Redis                                     |
| `routing.rs`  | Map service names to their respective Kafka/Redis instances             |

---

## Dependencies (`Cargo.toml`)

```toml
[dependencies]
actix-web = "4.11.0"
deadpool-redis = "0.22.0"
rdkafka = "0.38.0"
serde = "1.0.219"
serde_json = "1.0.143"
tokio = "1.47.1"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
```

---

## Running the Project

1. Clone the repo and enter the project:

```bash
git clone <repo-url>
cd micro-orchestrator
```

2. Build & run:

```bash
cargo run
```

Server runs at: `http://127.0.0.1:8080`

---

## HTTP Endpoints

### 1️⃣ Connect a Service

**Endpoint:** `POST /connect`

**Body:**

```json
{
  "service": "m-1",
  "kafka_broker": "127.0.0.1:9092",
  "redis_url": "redis://127.0.0.1:6379"
}
```

**Curl Example:**

```bash
curl -X POST http://127.0.0.1:8080/connect \
  -H "Content-Type: application/json" \
  -d '{"service":"m-1","kafka_broker":"127.0.0.1:9092","redis_url":"redis://127.0.0.1:6379"}'
```

**Response:**

```
Service m-1 registered
```

---

### 2️⃣ Check Service

**Endpoint:** `GET /check/{service}`

**Curl Example:**

```bash
curl http://127.0.0.1:8080/check/m-1
```

**Response (if Redis + Kafka OK):**

```
Redis: pong, Kafka: Kafka send OK
```
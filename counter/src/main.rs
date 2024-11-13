use rocket::{http::Status, State};
use rocket_db_pools::{
    deadpool_redis::{self, redis::AsyncCommands},
    Connection, Database,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("redis_pool")]
pub struct RedisPool(deadpool_redis::Pool);

struct AppState {
    count: AtomicUsize,
    hostname: String,
}

#[get("/count")]
fn get_count(app_state: &State<AppState>) -> String {
    let current_count = app_state.count.load(Ordering::Relaxed);

    format!("Hostname: {}, Count: {}", app_state.hostname, current_count)
}

#[get("/total")]
async fn get_total(mut redis: Connection<RedisPool>) -> String {
    let total: u64 = redis.get("total").await.unwrap_or(0);

    total.to_string()
}

#[post("/count")]
async fn increment_count(mut redis: Connection<RedisPool>, app_state: &State<AppState>) -> Status {
    app_state.count.fetch_add(1, Ordering::Relaxed);
    let _: () = redis.incr("total", 1).await.unwrap();

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            count: AtomicUsize::new(0),
            hostname: std::env::var("HOSTNAME").unwrap_or(String::from("Unknown")),
        })
        .attach(RedisPool::init())
        .mount("/", routes![get_count, increment_count, get_total])
}

// ROCKET_DATABASES='{redis_pool={url="redis://localhost:6379"}}' cargo run

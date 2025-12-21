use dotenv::dotenv;
mod crimson;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment
    dotenv().ok();
    
    // key names
    let central_db_instance_key = "CENTRAL_DATABASE_INSTANCE";
    let max_threads_key = "CRIMSON_MAX_THREADS";
    let redis_cluster_key = "REDIS_CLUSTER_INSTANCE";
    let crimson_hash_salt = "CRIMSON_HASH_SALT";

    // load keys
    let central_db_instance = std::env::var(central_db_instance_key)
        .expect("central database instance environment variable not set");
    let max_threads:u32 = std::env::var(max_threads_key)
        .expect("max threads environment variable not set")
        .parse()
        .expect("max threads environment variable was not an number");
    let redis_cluster_instance = std::env::var(redis_cluster_key)
        .expect("redis cluster instance environment variable not set");
    let crimson_hash_salt = std::env::var(crimson_hash_salt)
        .expect("hash salt environment variable not set");

    // sqlx pool
    let central_db_connection_pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_threads)
        .connect(&central_db_instance)
        .await
        .expect("connection to central db instance failed");
    
    // redis pool
    let mut deadpool_redis_config = deadpool_redis::Config::default();
    deadpool_redis_config.url = Some(redis_cluster_instance);

    let deadpool_redis_pool = deadpool_redis_config.create_pool(
        Some(deadpool_redis::Runtime::Tokio1)
    ).expect("failed to create redis pool");

    println!("[crimson]: created central db connection pool");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(crimson::server_types::ServerState {
                central_db_pool: central_db_connection_pool.clone(),
                redis_pool: deadpool_redis_pool.clone(),
                crimson_hash_salt: crimson_hash_salt.clone(),
                redis_expire_time: 86400,
                local_compute_ids: Vec::new(),
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

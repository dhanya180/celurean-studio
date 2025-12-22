use crate::crimson::api_auth_defs::http_get_user_register;
use tracing_loki::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod crimson;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment
    match dotenv::dotenv() {
        Ok(_) => eprintln!("[crimson]: loaded environment variables"),
        Err(e) => {
            eprint!("[crimson]: error loading dotenv ({})", e);
        }
    }

    // key names
    let central_db_instance_key = "CENTRAL_DATABASE_INSTANCE";
    let max_threads_key = "CRIMSON_MAX_THREADS";
    let redis_cluster_key = "REDIS_CLUSTER_INSTANCE";
    let loki_url_key = "LOKI_URL";
    let crimson_hash_salt_key = "CRIMSON_HASH_SALT";
    let trace_level_key = "TRACE_LEVEL";

    // load keys
    let central_db_instance = match std::env::var(central_db_instance_key) {
        Ok(var) => var,
        Err(e) => {
            panic!(
                "[crimson]: missing environment variable {} | ({})",
                central_db_instance_key, e
            );
        }
    };

    let max_threads: u32 = match std::env::var(max_threads_key) {
        Ok(var) => match var.parse() {
            Ok(var_u32) => var_u32,
            Err(e) => {
                panic!(
                    "[crimson]: environment variable {} is not an integer | ({})",
                    max_threads_key, e
                )
            }
        },
        Err(e) => {
            panic!(
                "[crimson]: missing environment variable {} | ({})",
                max_threads_key, e
            );
        }
    };

    let redis_cluster_instance = match std::env::var(redis_cluster_key) {
        Ok(var) => var,
        Err(e) => {
            panic!(
                "[crimson]: missing environment variable {} | ({})",
                redis_cluster_key, e
            );
        }
    };

    let loki_url = match std::env::var(loki_url_key) {
        Ok(var) => var,
        Err(e) => {
            panic!(
                "[crimson]: missing environment variable {} | ({})",
                loki_url_key, e
            );
        }
    };

    let crimson_hash_salt = match std::env::var(crimson_hash_salt_key) {
        Ok(var) => var,
        Err(e) => {
            panic!(
                "[crimson]: missing environment variable {} | ({})",
                crimson_hash_salt_key, e
            );
        }
    };

    let trace_level = match std::env::var(trace_level_key) {
        Ok(var) => var,
        Err(e) => {
            panic!(
                "[crimson]: missing environment variable {} | ({})",
                trace_level_key, e
            );
        }
    };

    // sqlx pool allocation
    let central_db_connection_pool: sqlx::Pool<sqlx::Postgres> =
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(max_threads)
            .connect(&central_db_instance)
            .await
        {
            Ok(connection_pool) => {
                eprintln!("[crimson]: central db connection pool created");
                connection_pool
            }
            Err(e) => {
                panic!(
                    "[crimson]: central db connection pool creation failed | ({})",
                    e
                )
            }
        };

    // redis pool allocation
    let deadpool_redis_config = deadpool_redis::Config::from_url(&redis_cluster_instance);

    let deadpool_redis_pool =
        match deadpool_redis_config.create_pool(Some(deadpool_redis::Runtime::Tokio1)) {
            Ok(redis_pool) => {
                eprintln!("[crimson]: redis pool connection created");
                redis_pool
            }
            Err(e) => {
                panic!("[crimson]: redis pool creation failed | ({})", e);
            }
        };

    // tracing subscriber & promethus initialisation
    let (loki_layer, task) = tracing_loki::builder()
        .label("service", "crimson_heart")
        .unwrap()
        .build_url(url::Url::parse(&loki_url).unwrap())
        .unwrap();

    tokio::spawn(task);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(trace_level))
        .with(tracing_subscriber::fmt::layer())
        .with(loki_layer)
        .init();

    let prometheus_instance = match actix_web_prom::PrometheusMetricsBuilder::new("crimson")
        .endpoint("/metrics")
        .build()
    {
        Ok(instance) => {
            eprintln!("[crimson]: prometheus instance created");
            instance
        }
        Err(e) => {
            panic!("[crimson]: prometheus instance creation failed | ({})", e);
        }
    };

    // spin up the server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(prometheus_instance.clone())
            .app_data(actix_web::web::Data::new(
                crimson::server_types::ServerState {
                    central_db_pool: central_db_connection_pool.clone(),
                    redis_pool: deadpool_redis_pool.clone(),
                    crimson_hash_salt: crimson_hash_salt.clone(),
                    redis_expire_time: 86400,
                    local_compute_ids: Vec::new(),
                },
            ))
            .service(http_get_user_register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

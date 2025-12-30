use super::api_auth_types;
use super::server_types;
use crate::crimson::server_types::SessionUserState;

use argon2::PasswordVerifier;
use argon2::password_hash::PasswordHasher;
use deadpool_redis::redis::AsyncCommands;

/**
 * # Brief
 * HTTP POST request. Registers the User in Central DB.
 *
 * # Detail
 * - Validates the Redis Pool Connection, returns InternalServerError on Failure.
 * - Uses `session_id` Cookie to manage sessions, checks for it in the Cookie,
 * if the Cookie value isn't available, creates a new `session_id` and adds
 * to redis and creates the Cookie.
 * - Writes to Central Database if user is unregistered.
 *
*/
#[actix_web::post("/auth/register")]
async fn http_get_user_register(
    __request_metadata: actix_web::HttpRequest,
    __request_payload: actix_web::web::Json<api_auth_types::HTTPUserRegister>,
    __server_state: actix_web::web::Data<server_types::ServerState>,
) -> impl actix_web::Responder {
    // get the redis connection pool
    let mut redis_connection = match __server_state.redis_pool.get().await {
        Ok(redis_connection) => {
            tracing::debug!(component = "redis_pool", "Fetched redis connection");
            redis_connection
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_connection_pool",
                "failed to acquire VAR `redis_connection` from actix_web's thread"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    let mut full_site_cookie: Option<actix_web::cookie::Cookie> = None;
    let session_id = match __request_metadata
        .cookie("session_id")
        .map(|c| c.value().to_string())
    {
        // Cookie `session_id` already exists
        Some(id) => id,
        // Cookie `session_id` doesnt exist
        None => {
            // Session ID Creation & Updating in Redis

            let new_session_id = uuid::Uuid::now_v7().to_string();
            let session_key = format!("session_id:{}", new_session_id);
            let _: () = match redis_connection
                .hset_multiple(
                    &session_key,
                    &[("state", SessionUserState::Anonymous.as_u32().to_string())],
                )
                .await
            {
                Ok(redis_value) => redis_value,
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        component = "redis_functions",
                        function = "hset_multiple",
                        "function failed & returned error"
                    );
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Server Error, Refresh and Retry\n");
                }
            };

            let _: () = match redis_connection
                .expire(&session_key, __server_state.redis_expire_time)
                .await
            {
                Ok(redis_value) => redis_value,
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        component = "redis_functions",
                        function = "expire",
                        "function failed & returned error"
                    );
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Server Error, Refresh and Retry\n");
                }
            };

            // Set Cookie
            full_site_cookie = Some(
                actix_web::cookie::Cookie::build("session_id", new_session_id.clone())
                    .path("/")
                    .max_age(actix_web::cookie::time::Duration::seconds(
                        __server_state.redis_expire_time as i64,
                    ))
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .http_only(true)
                    .finish(),
            );

            new_session_id
        }
    };

    let session_key = format!("session_id:{}", &session_id);
    let user_state: u32 = match redis_connection.hget(&session_key, "state").await {
        Ok(Some(state)) => state,
        Ok(None) => {
            tracing::info!(
                component = "user_state",
                "invalid session, `session_key` from cookie was not found in redis instance"
            );
            return actix_web::HttpResponse::BadRequest()
                .body("Invalid Session, Refresh & Retry\n");
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_functions",
                function = "hget",
                "function failed & returned error"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    if user_state == SessionUserState::Registered.as_u32() {
        tracing::info!(component = "user_state", "user tried to register twice");
        return actix_web::HttpResponse::Conflict().body("You are already registered\n");
    } else {
        // Write to Central DB
        let user_id = uuid::Uuid::now_v7().to_string();
        let username = &__request_payload.username;
        let password_string = &__request_payload.password;
        let email = &__request_payload.email;
        let birth_date = &__request_payload.birth_date;
        let user_salt = argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        );

        let password_string_clone = password_string.clone();
        let user_salt_clone = user_salt.clone();

        // hash the password
        // asynchronous version
        let password = actix_web::web::block(move || {
            argon2::Argon2::default()
                .hash_password(password_string_clone.as_bytes(), &user_salt_clone)
                .map(|hash| hash.to_string())
        })
        .await
        .unwrap();

        let password = match password {
            Ok(hash) => {
                tracing::info!(
                    component = "generic",
                    function = "argon2_hashing",
                    "hashed password for user"
                );
                hash.to_string()
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    component = "generic",
                    function = "argon2_hashing",
                    "function failed & returned error"
                );
                return actix_web::HttpResponse::InternalServerError()
                    .body("Server Error, Refresh & Retry\n");
            }
        };

        let sqlx_insert_query = r#"
            INSERT INTO users 
            (user_id, username, password, email, birth_date, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING user_id;
        "#;

        // insert
        match sqlx::query(sqlx_insert_query)
            .bind(user_id)
            .bind(username)
            .bind(password)
            .bind(email)
            .bind(birth_date)
            .fetch_one(&__server_state.central_db_pool)
            .await
        {
            Ok(_) => {
                tracing::info!(
                    component = "database",
                    query = "INSERT INTO",
                    table = "users",
                    "user registration was added"
                );
            }
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    // user already exists
                    if db_err.is_unique_violation() {
                        tracing::error!(
                            error = %e,
                            component = "database",
                            query = "INSERT INTO",
                            table = "users",
                            "user already exists"
                        );
                        return actix_web::HttpResponse::Conflict()
                            .body(format!("Email {} already registered\n", email));
                    }
                }
                tracing::error!(
                    error = %e,
                    component = "database",
                    query = "NONE",
                    table = "users",
                    "function failed & returned error"
                );
                return actix_web::HttpResponse::InternalServerError()
                    .body("Server Error, Refresh & Retry\n");
            }
        };

        let _: () = match redis_connection
            .hset(
                &session_key,
                "state",
                SessionUserState::Registered.as_u32().to_string(),
            )
            .await
        {
            Ok(redis_value) => redis_value,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    component = "redis_functions",
                    function = "hset",
                    "function failed & returned error"
                );
                return actix_web::HttpResponse::InternalServerError()
                    .body("Server Error, Refresh and Retry\n");
            }
        };
    }

    match full_site_cookie {
        Some(cookie) => {
            return actix_web::HttpResponse::Ok()
                .cookie(cookie)
                .body("successful\n");
        }
        None => {
            tracing::info!(component = "cookie", "user cookie already exists");
            return actix_web::HttpResponse::Ok().body("successful\n");
        }
    }
}

/**
 * # Brief
 * HTTP POST request. Logs in the User.
 *
 * # Detail
 * - Validates Redis Pool Connection.
 * - Uses `session_id` Cookie to manage sessions.
 * - Verifies password using Argon2.
 * - Updates Redis session state on success.
 *
*/
#[actix_web::post("/auth/login")]
async fn http_post_user_login(
    __request_metadata: actix_web::HttpRequest,
    __request_payload: actix_web::web::Json<api_auth_types::HTTPUserLogin>,
    __server_state: actix_web::web::Data<server_types::ServerState>,
) -> impl actix_web::Responder {
    // get the redis connection pool
    let mut redis_connection = match __server_state.redis_pool.get().await {
        Ok(redis_connection) => {
            tracing::debug!(component = "redis_pool", "Fetched redis connection");
            redis_connection
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_connection_pool",
                "failed to acquire VAR `redis_connection` from actix_web's thread"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    let mut full_site_cookie: Option<actix_web::cookie::Cookie> = None;

    let session_id = match __request_metadata
        .cookie("session_id")
        .map(|c| c.value().to_string())
    {
        Some(id) => id,
        None => {
            let new_session_id = uuid::Uuid::now_v7().to_string();
            let session_key = format!("session_id:{}", new_session_id);

            let _: () = match redis_connection
                .hset_multiple(
                    &session_key,
                    &[("state", SessionUserState::Anonymous.as_u32().to_string())],
                )
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        component = "redis_functions",
                        function = "hset_multiple",
                        "function failed & returned error"
                    );
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Server Error, Refresh and Retry\n");
                }
            };

            let _: () = match redis_connection
                .expire(&session_key, __server_state.redis_expire_time)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        component = "redis_functions",
                        function = "expire",
                        "function failed & returned error"
                    );
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Server Error, Refresh and Retry\n");
                }
            };

            full_site_cookie = Some(
                actix_web::cookie::Cookie::build("session_id", new_session_id.clone())
                    .path("/")
                    .max_age(actix_web::cookie::time::Duration::seconds(
                        __server_state.redis_expire_time as i64,
                    ))
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .http_only(true)
                    .finish(),
            );

            new_session_id
        }
    };

    let session_key = format!("session_id:{}", &session_id);

    // fetch user from DB
    let identity = &__request_payload.identity;
    let password = &__request_payload.password;

    let user = match sqlx::query!(
        r#"
        SELECT password
        FROM users
        WHERE email = $1 OR username = $1
        "#,
        identity
    )
    .fetch_optional(&__server_state.central_db_pool)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return actix_web::HttpResponse::NotFound()
                .body("User not registered\n");
        }

        Err(e) => {
            tracing::error!(
                error = %e,
                component = "database",
                query = "SELECT",
                table = "users",
                "function failed & returned error"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    // verify password
    let parsed_hash = match argon2::password_hash::PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "argon2",
                "failed to parse password hash"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    if argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return actix_web::HttpResponse::Unauthorized()
            .body("Invalid credentials\n");
    }

    // update session state
    let _: () = match redis_connection
        .hset(
            &session_key,
            "state",
            SessionUserState::Registered.as_u32().to_string(),
        )
        .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_functions",
                function = "hset",
                "function failed & returned error"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    match full_site_cookie {
        Some(cookie) => {
            actix_web::HttpResponse::Ok()
                .cookie(cookie)
                .body("successful\n")
        }
        None => {
            tracing::info!(component = "cookie", "user cookie already exists");
            actix_web::HttpResponse::Ok().body("successful\n")
        }
    }
}

/**
 * # Brief
 * HTTP POST request. Logs out the User.
 *
 * # Detail
 * - Invalidates existing Redis session.
 * - Generates a new session_id.
 * - Marks new session as Anonymous.
 * - Issues a fresh HttpOnly cookie.
 */
#[actix_web::post("/auth/logout")]
async fn http_post_user_logout(
    __request_metadata: actix_web::HttpRequest,
    __server_state: actix_web::web::Data<server_types::ServerState>,
) -> impl actix_web::Responder {
    // get redis connection
    let mut redis_connection = match __server_state.redis_pool.get().await {
        Ok(conn) => {
            tracing::debug!(component = "redis_pool", "Fetched redis connection");
            conn
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_connection_pool",
                "failed to acquire redis connection"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    // check if a session already exists
    if let Some(cookie) = __request_metadata.cookie("session_id") {
        let old_session_id = cookie.value();
        let old_session_key = format!("session_id:{}", old_session_id);

        // delete old session from redis (hard invalidation)
        let _: () = match redis_connection.del(&old_session_key).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    component = "redis_functions",
                    function = "del",
                    "failed to delete old session"
                );
                return actix_web::HttpResponse::InternalServerError()
                    .body("Server Error, Refresh & Retry\n");
            }
        };
    }

    // generate new session
    let new_session_id = uuid::Uuid::now_v7().to_string();
    let new_session_key = format!("session_id:{}", new_session_id);

    // mark new session as anonymous
    let _: () = match redis_connection
        .hset_multiple(
            &new_session_key,
            &[("state", SessionUserState::Anonymous.as_u32().to_string())],
        )
        .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_functions",
                function = "hset_multiple",
                "failed to create new anonymous session"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    // set TTL
    let _: () = match redis_connection
        .expire(&new_session_key, __server_state.redis_expire_time)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(
                error = %e,
                component = "redis_functions",
                function = "expire",
                "failed to set session expiry"
            );
            return actix_web::HttpResponse::InternalServerError()
                .body("Server Error, Refresh & Retry\n");
        }
    };

    // issue new cookie
    let cookie = actix_web::cookie::Cookie::build("session_id", new_session_id)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::seconds(
            __server_state.redis_expire_time as i64,
        ))
        .same_site(actix_web::cookie::SameSite::Lax)
        .http_only(true)
        .finish();

    actix_web::HttpResponse::Ok()
        .cookie(cookie)
        .body("logged out\n")
}


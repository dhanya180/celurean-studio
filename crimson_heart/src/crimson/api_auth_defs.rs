use deadpool_redis::redis::AsyncCommands;

use crate::crimson::server_types::SessionUserState;

use super::api_auth_types;
use super::server_types;

/**
 * POST request, registers the user
*/
#[actix_web::post("/auth/register")]
async fn http_get_user_register(
    __request_metadata: actix_web::HttpRequest,
    __request_payload: actix_web::web::Json<api_auth_types::HTTPUserRegister>,
    __server_state: actix_web::web::Data<server_types::ServerState>
) -> impl actix_web::Responder {  
    
    let request_payload = __request_payload.into_inner();
    let connection_info = __request_metadata.connection_info();
    
    let mut redis_connection = match __server_state.redis_pool.get().await {
        Ok(redis_connection) => redis_connection,
        Err(e) =>{
            return actix_web::HttpResponse::InternalServerError().body(format!("fetching redis connection failed"));
        }
    };  

    let mut session_id = match __request_metadata.cookie("session_id")
    .map(|c| c.to_string()){
        Some(id) => id,
        None => {
            let new_session_id = uuid::Uuid::now_v7().to_string();
            redis_connection.hset_multiple(&new_session_id, 
                &[
                    ("state", SessionUserState::Anonymous.as_u32())
                ]
            );
            redis_connection.expire(&new_session_id, __server_state.redis_expire_time);
            new_session_id
        }
    };

    let user_state: u32 =  match redis_connection.hget(&session_id, "state").await {
        Ok(Some(state)) => state,
        Ok(None) => {
            SessionUserState::Zombie.as_u32()
        },
        Err(e) =>{
            return actix_web::HttpResponse::InternalServerError().body(format!("{}", e));
        }
    };

    return actix_web::HttpResponse::Ok().body("");
}
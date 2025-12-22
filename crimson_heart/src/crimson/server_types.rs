use sqlx::{Pool, Postgres};

pub struct ServerState {
    pub central_db_pool: Pool<Postgres>,
    pub redis_pool: deadpool_redis::Pool,
    pub local_compute_ids: Vec<String>,
    pub crimson_hash_salt: String,
    pub redis_expire_time: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum SessionUserState {
    Anonymous,
    Registered,
}

impl SessionUserState {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            1 => Some(SessionUserState::Anonymous),
            2 => Some(SessionUserState::Registered),
            _ => None,
        }
    }
}

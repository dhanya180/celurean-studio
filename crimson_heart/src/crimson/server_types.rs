use sqlx::{Pool, Postgres};

pub struct ServerState {
    pub central_db_pool: Pool<Postgres>,
    pub redis_pool: deadpool_redis::Pool,
    pub local_compute_ids: Vec<String>,
    pub crimson_hash_salt: String,
    pub redis_expire_time: i64,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum SessionUserState {
    Anonymous  = 1,
    Registered = 2,
}

impl SessionUserState {
    #[inline]
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    #[inline]
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            1 => Some(SessionUserState::Anonymous),
            2 => Some(SessionUserState::Registered),
            _ => None,
        }
    }
}

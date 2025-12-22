use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPUserRegister {
    pub username: String,
    pub password: String,
    pub birth_date: String,
    pub email: String,
}

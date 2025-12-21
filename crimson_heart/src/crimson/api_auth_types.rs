use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPUserRegister {
    pub username: String,
    pub birth_date: String,
    pub email: String
}




use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamAccount {
    id: i32,
    username: String,
    email: String,
    balance: f32
}
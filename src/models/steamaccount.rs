use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamAccount {
    id: i32,
    email: String,
    username: String,
    balance: f32
}
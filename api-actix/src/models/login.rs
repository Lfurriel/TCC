use serde::{Deserialize, Serialize};
use crate::models::cliente::ClienteResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub cliente: ClienteResponse,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub cliente: String,
    pub exp: usize,
}
pub struct AuthConfig {
    pub secret: String,
    pub expires_in: i64,
}

impl AuthConfig {
    pub fn new() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "seu_jwt_secret_aqui".to_string()),
            expires_in: 86400,
        }
    }
}
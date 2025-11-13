use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppMessage {
    pub status_code: u16,
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl AppMessage {
    pub fn new(message: &str, status_code: u16) -> Self {
        let status = if status_code >= 200 && status_code <= 299 {
            "success".to_string()
        } else {
            "error".to_string()
        };

        Self {
            status_code,
            status,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn response<T: serde::Serialize>(message: &str, status_code: u16, data: T) -> HttpResponse {
        let status = if status_code >= 200 && status_code <= 299 {
            "success".to_string()
        } else {
            "error".to_string()
        };

        let response = serde_json::json!({
            "statusCode": status_code,
            "status": status,
            "message": message,
            "data": data
        });

        let http_status = actix_web::http::StatusCode::from_u16(status_code)
            .unwrap_or(actix_web::http::StatusCode::OK);

        HttpResponse::build(http_status).json(response)
    }

    pub fn with_data(message: &str, status_code: u16, data: serde_json::Value) -> Self {
        let mut error = Self::new(message, status_code);
        error.data = Some(data);
        error
    }
}

impl fmt::Display for AppMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for AppMessage {
    fn error_response(&self) -> HttpResponse {
        let status_code = actix_web::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        let error_response = serde_json::json!({
            "statusCode": self.status_code,
            "status": self.status,
            "message": self.message,
            "data": self.data,
        });

        HttpResponse::build(status_code)
            .json(error_response)
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub status: String,
    pub message: String,
    pub errors: Vec<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ValidationError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

#[derive(Debug)]
pub enum ApiError {
    App(AppMessage),
    Validation(ValidationError),
    Internal(anyhow::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::App(e) => write!(f, "{}", e),
            Self::Validation(e) => write!(f, "{}", e.message),
            Self::Internal(e) => write!(f, "{}", e),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::App(e) => e.error_response(),
            Self::Validation(e) => e.error_response(),
            Self::Internal(_) => {
                let error = serde_json::json!({
                    "status": "error",
                    "message": "Erro interno"
                });
                HttpResponse::InternalServerError().json(error)
            }
        }
    }
}

impl From<AppMessage> for ApiError {
    fn from(error: AppMessage) -> Self {
        Self::App(error)
    }
}

impl From<ValidationError> for ApiError {
    fn from(error: ValidationError) -> Self {
        Self::Validation(error)
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal(error)
    }
}

// Adicione isso ao arquivo errors.rs
impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => {
                AppMessage::new("Recurso não encontrado", 404).into()
            }
            _ => {
                let message = format!("Erro de banco de dados: {}", error);
                Self::Internal(anyhow::anyhow!(message))
            }
        }
    }
}

pub fn success_response<T: Serialize>(message: &str, status_code: u16, data: T) -> HttpResponse {
    let status = if status_code >= 200 && status_code <= 299 {
        "success"
    } else {
        "error"
    };

    let response = serde_json::json!({
        "statusCode": status_code,
        "status": status,
        "message": message,
        "data": data
    });

    let http_status = actix_web::http::StatusCode::from_u16(status_code)
        .unwrap_or(actix_web::http::StatusCode::OK);

    HttpResponse::build(http_status).json(response)
}
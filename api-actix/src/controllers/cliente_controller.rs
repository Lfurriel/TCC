use actix_web::{post, web, HttpResponse, Result};
use crate::services::cliente_service::{ClienteService, LoginPayload};
use crate::utils::app_message::{success_response, ApiError};
use crate::db::AppState;
use crate::models::cliente::CreateClientePayload;

#[post("")]
async fn create(
    app_state: web::Data<AppState>,
    payload: web::Json<CreateClientePayload>
) -> Result<HttpResponse, ApiError> {
    let result = ClienteService::create(&app_state.db_pool, payload.into_inner()).await?;
    Ok(success_response("Cliente criado com sucesso", 201, result))
}

#[post("/login")]
async fn login(
    app_state: web::Data<AppState>,
    payload: web::Json<LoginPayload>
) -> Result<HttpResponse, ApiError> {
    let result = ClienteService::login(&app_state.db_pool, payload.into_inner()).await?;
    Ok(success_response("Login realizado com sucesso", 200, result))
}
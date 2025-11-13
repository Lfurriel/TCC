use actix_web::{get, web, HttpResponse};
use crate::services::home_service::HomeService;
use crate::utils::app_message::{success_response, ApiError};
use crate::db::AppState;

#[get("/amazon")]
async fn get_home_amazon(app_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let results = HomeService::get_home_amazon(&app_state.db_pool).await?;
    Ok(success_response("Home obtida com sucesso", 200, results))
}

#[get("/shopee")]
async fn get_home_shopee(app_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let results = HomeService::get_home_shopee(&app_state.db_pool).await?;
    Ok(success_response("Home obtida com sucesso", 200, results))
}
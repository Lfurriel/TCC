use actix_web::{get, web, HttpResponse, Result};
use crate::services::categoria_service::CategoriaService;
use crate::utils::app_message::{success_response, ApiError};
use crate::db::AppState;

#[get("")]
async fn get_all(app_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let categorias = CategoriaService::get_all(&app_state.db_pool).await?;
    Ok(success_response("Categorias obtidas com sucesso", 200, categorias))
}
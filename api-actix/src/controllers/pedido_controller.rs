use actix_web::{post, web, HttpResponse, Result, HttpRequest};
use serde_json::Value;
use crate::services::pedido_service::PedidoService;
use crate::utils::app_message::{success_response, ApiError};
use crate::middlewares::is_authenticated::get_cliente_from_request;
use crate::db::AppState;

#[post("")]
async fn create(
    app_state: web::Data<AppState>,
    payload: web::Json<Value>,
    req: HttpRequest
) -> Result<HttpResponse, ApiError> {
    let cliente = get_cliente_from_request(&req)?;
    let mut pedido_payload = payload.into_inner();

    if let Some(obj) = pedido_payload.as_object_mut() {
        obj.insert("idCliente".to_string(), Value::String(cliente.id));
    }

    let result = PedidoService::create(&app_state.db_pool, pedido_payload).await?;
    Ok(success_response("Pedido criado com sucesso", 201, result))
}

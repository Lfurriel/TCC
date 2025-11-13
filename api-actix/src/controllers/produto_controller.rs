use actix_web::{get, web, HttpResponse};
use crate::services::produto_service::ProdutoService;
use crate::utils::app_message::{success_response, ApiError, AppMessage};
use crate::db::AppState;
use crate::models::produto::{QueryParams, QueryParamsWithName};

#[get("")]
async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<QueryParams>
) -> Result<HttpResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let results = ProdutoService::get_all(&app_state.db_pool, page, page_size).await?;
    Ok(success_response("Produtos obtidos com sucesso", 200, results))
}

#[get("/{sku}")]
async fn get_by_sku(
    app_state: web::Data<AppState>,
    path: web::Path<String>
) -> Result<HttpResponse, ApiError> {
    let sku = path.into_inner();

    let produto = ProdutoService::get_by_sku(&app_state.db_pool, &sku).await?;
    Ok(success_response("Produto obtido com sucesso", 200, produto))
}

#[get("/categoria/{id_categoria}")]
async fn get_by_categoria(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<QueryParams>
) -> Result<HttpResponse, ApiError> {
    let id_categoria = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let produtos = ProdutoService::get_by_id_categoria(&app_state.db_pool, &id_categoria, page, page_size).await?;
    Ok(success_response("Produtos da categoria obtidos com sucesso", 200, produtos))
}

#[get("/ofertas")]
async fn get_ofertas(
    app_state: web::Data<AppState>,
    query: web::Query<QueryParams>
) -> Result<HttpResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let ofertas = ProdutoService::get_all_ofertas(&app_state.db_pool, page, page_size).await?;
    Ok(success_response("Ofertas obtidas com sucesso", 200, ofertas))
}

#[get("/destaques")]
async fn get_destaques(
    app_state: web::Data<AppState>,
    query: web::Query<QueryParams>
) -> Result<HttpResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let destaques = ProdutoService::get_all_destaques(&app_state.db_pool, page, page_size).await?;
    Ok(success_response("Produtos em destaque obtidos com sucesso", 200, destaques))
}

#[get("/nome")]
async fn get_by_nome(
    app_state: web::Data<AppState>,
    query: web::Query<QueryParamsWithName>
) -> Result<HttpResponse, ApiError> {
    let name = query.name.as_ref()
        .ok_or_else(|| AppMessage::new(&*"Parâmetro 'name' é obrigatório".to_string(), 400))?;

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let produtos = ProdutoService::get_by_nome(&app_state.db_pool, name, page, page_size).await?;
    Ok(success_response("Produtos encontrados com sucesso", 200, produtos))
}
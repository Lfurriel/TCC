use crate::dal::produto_dal::ProdutoDal;
use crate::utils::app_message::ApiError;
use crate::db::DbPool;
use crate::models::produto::Produto;

pub struct ProdutoService;

impl ProdutoService {
    pub async fn get_all(pool: &DbPool, page: u32, page_size: u32) -> Result<Vec<Produto>, ApiError> {
        ProdutoDal::get_all(pool, "*", page, page_size).await
    }

    pub async fn get_by_sku(pool: &DbPool, sku: &String) -> Result<serde_json::Value, ApiError> {
        ProdutoDal::get_by_sku(pool, sku, "sku, produtos.nome, foto, pctOferta, preco, descricao, estoque").await
    }

    pub async fn get_by_id_categoria(pool: &DbPool, id_categoria: &String, page: u32, page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        ProdutoDal::get_by_id_categoria(pool, id_categoria, "sku, produtos.nome, foto, pctOferta, preco, qtdVendas", page, page_size).await
    }

    pub async fn get_all_ofertas(pool: &DbPool, page: u32, page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        ProdutoDal::get_all_ofertas(pool, "sku, produtos.nome, foto, pctOferta, preco", page, page_size).await
    }

    pub async fn get_all_destaques(pool: &DbPool, page: u32, page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        ProdutoDal::get_all_destaques(pool, "sku, produtos.nome, foto, pctOferta, preco", page, page_size).await
    }

    pub async fn get_by_nome(pool: &DbPool, nome: &String, page: u32, page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        ProdutoDal::get_by_nome(pool, nome, "sku, produtos.nome, foto, pctOferta, preco", page, page_size).await
    }
}
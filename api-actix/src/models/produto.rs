use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::produtos;

#[derive(Deserialize)]
pub struct ProdutoPayload {
    #[serde(rename = "skuProduto")]
    pub sku_produto: String,
    pub quantidade: i32,
    #[serde(rename = "valorUnitario")]
    pub valor_unitario: f64,
    #[serde(rename = "valorBruto")]
    pub valor_bruto: f64,
    #[serde(rename = "valorDesconto")]
    pub valor_desconto: f64,
    #[serde(rename = "valorLiquido")]
    pub valor_liquido: f64,
    #[serde(rename = "valorFrete")]
    pub valor_frete: f64,
}

#[derive(Queryable, QueryableByName, Debug, Identifiable, Serialize, Selectable, Deserialize, Insertable)]
#[diesel(table_name = produtos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
#[diesel(primary_key(sku))]
pub struct Produto {
    pub sku: String,
    pub codigo: i32,
    #[diesel(column_name = "idCategoria")]
    pub id_categoria: String,
    pub nome: String,
    pub descricao: Option<String>,
    pub foto: Option<String>,
    pub preco: BigDecimal,
    pub estoque: BigDecimal,
    pub pctoferta: BigDecimal,
    pub qtdvendas: i32,
    #[diesel(column_name = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct QueryParamsWithName {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
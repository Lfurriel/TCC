use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::models::produto::ProdutoPayload;
pub(crate) use crate::schema::pedidos;
use crate::schema::enderecosEntrega as enderecos_entregas;
use crate::schema::pagamentos;
use crate::schema::produtosPedido as produtos_pedidos;

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct EnderecosEntrega {
    pub id: String,
    #[diesel(column_name = "idPedido")]
    #[serde(rename = "idPedido")]
    pub id_pedido: String,
    #[diesel(column_name = "nomeRemetente")]
    #[serde(rename = "nomeRemetente")]
    pub nome_remetente: String,
    pub cep: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    #[diesel(column_name = "codigoIbgeCidade")]
    #[serde(rename = "codigoIbgeCidade")]
    pub codigo_ibge_cidade: String,
    #[diesel(column_name = "codigoIbgeUF")]
    #[serde(rename = "codigoIbgeUF")]
    pub codigo_ibge_uf: String,
    #[diesel(column_name = "createdAt")]
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[serde(rename_all = "camelCase")]
pub struct Pagamento {
    pub id: String,
    #[diesel(column_name = "idPedido")]
    pub id_pedido: String,
    #[diesel(column_name = "formaPagamento")]
    #[serde(rename = "formaPagamento")]
    pub forma_pagamento: String,
    #[diesel(column_name = "numeroParcelas")]
    #[serde(rename = "numeroParcelas")]
    pub numero_parcelas: i16,
    #[diesel(column_name = "valorParcela")]
    #[serde(rename = "valorParcela")]
    pub valor_parcela: BigDecimal,
    #[diesel(column_name = "valorTotal")]
    #[serde(rename = "valorTotal")]
    pub valor_total: BigDecimal,
    pub boleto: Option<String>,
    pub pix: Option<String>,
    pub tid: Option<String>,
    #[diesel(column_name = "createdAt")]
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[serde(rename_all = "camelCase")]
pub struct Pedido {
    pub id: String,
    #[diesel(column_name = "idCliente")]
    pub id_cliente: String,
    #[diesel(column_name = "valorBruto")]
    pub valor_bruto: BigDecimal,
    #[diesel(column_name = "valorFrete")]
    pub valor_frete: BigDecimal,
    #[diesel(column_name = "valorDesconto")]
    pub valor_desconto: BigDecimal,
    #[diesel(column_name = "valorLiquido")]
    pub valor_liquido: BigDecimal,
    pub status: String,
    #[diesel(column_name = "dataEntrega")]
    pub data_entrega: NaiveDate,
    #[diesel(column_name = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Selectable, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[serde(rename_all = "camelCase")]
pub struct ProdutosPedido {
    pub id: String,
    #[diesel(column_name = "idPedido")]
    pub id_pedido: String,
    #[diesel(column_name = "skuProduto")]
    pub sku_produto: String,
    pub quantidade: BigDecimal,
    #[diesel(column_name = "valorUnitario")]
    pub valor_unitario: BigDecimal,
    #[diesel(column_name = "valorBruto")]
    pub valor_bruto: BigDecimal,
    #[diesel(column_name = "valorFrete")]
    pub valor_frete: BigDecimal,
    #[diesel(column_name = "valorDesconto")]
    pub valor_desconto: BigDecimal,
    #[diesel(column_name = "valorLiquido")]
    pub valor_liquido: BigDecimal,
    #[diesel(column_name = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct EnderecoPayload {
    #[serde(rename = "nomeRemetente")]
    pub nome_remetente: String,
    pub cep: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    #[serde(rename = "codigoIbgeCidade")]
    pub codigo_ibge_cidade: String,
    #[serde(rename = "codigoIbgeUF")]
    pub codigo_ibge_uf: String,
}

#[derive(Deserialize)]
pub struct PagamentoPayload {
    #[serde(rename = "formaPagamento")]
    pub forma_pagamento: String,
    #[serde(rename = "numeroParcelas")]
    pub numero_parcelas: i32,
    #[serde(rename = "valorTotal")]
    pub valor_total: f64,
    #[serde(rename = "valorParcela")]
    pub valor_parcela: f64,
}

#[derive(Deserialize)]
pub struct PedidoPayload {
    #[serde(rename = "idCliente")]
    pub id_cliente: String,
    #[serde(rename = "valorBruto")]
    pub valor_bruto: f64,
    #[serde(rename = "valorDesconto")]
    pub valor_desconto: f64,
    #[serde(rename = "valorFrete")]
    pub valor_frete: f64,
    #[serde(rename = "valorLiquido")]
    pub valor_liquido: f64,
    #[serde(rename = "dataEntrega")]
    pub data_entrega: String,
    #[serde(rename = "enderecoEntrega")]
    pub endereco_entrega: EnderecoPayload,
    pub pagamento: PagamentoPayload,
    pub produtos: Vec<ProdutoPayload>,
}
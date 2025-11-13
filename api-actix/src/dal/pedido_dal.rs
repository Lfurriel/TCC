use crate::db::DbPool;
use crate::schema::{enderecosEntrega, pagamentos, pedidos, produtosPedido};
use crate::utils::app_message::{ApiError, AppMessage};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;
use crate::models::pedido::{EnderecosEntrega, Pagamento, Pedido, PedidoPayload, ProdutosPedido};

pub struct PedidoDal;

impl PedidoDal {
    pub async fn create(pool: &DbPool, payload: PedidoPayload) -> Result<Value, ApiError> {
        let pool_clone = pool.clone();
        let payload_owned = payload;

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            let PedidoPayload {
                endereco_entrega,
                pagamento,
                produtos,
                id_cliente,
                valor_bruto,
                valor_desconto,
                valor_frete,
                valor_liquido,
                data_entrega,
            } = payload_owned;

            let valor_bruto_bd = BigDecimal::from_str(&valor_bruto.to_string())
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

            let valor_desconto_bd = BigDecimal::from_str(&valor_desconto.to_string())
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

            let valor_frete_bd = BigDecimal::from_str(&valor_frete.to_string())
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

            let valor_liquido_bd = BigDecimal::from_str(&valor_liquido.to_string())
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

            let data_entrega_naive = chrono::NaiveDateTime::parse_from_str(&data_entrega, "%Y-%m-%dT%H:%M:%SZ")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(&data_entrega, "%Y-%m-%dT%H:%M:%S%.fZ"))
                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

            let data_entrega_date = data_entrega_naive.date();

            connection
                .transaction(|conn| {
                    let id_pedido = Uuid::new_v4().to_string();

                    let pedido = diesel::insert_into(pedidos::table)
                        .values((
                            pedidos::id.eq(&id_pedido),
                            pedidos::idCliente.eq(&id_cliente),
                            pedidos::valorBruto.eq(valor_bruto_bd),
                            pedidos::valorDesconto.eq(valor_desconto_bd),
                            pedidos::valorFrete.eq(valor_frete_bd),
                            pedidos::valorLiquido.eq(valor_liquido_bd),
                            pedidos::dataEntrega.eq(data_entrega_date),
                            pedidos::status.eq("P"),
                            pedidos::createdAt.eq(diesel::dsl::now),
                            pedidos::updatedAt.eq(diesel::dsl::now),
                        ))
                        .get_result::<Pedido>(conn)?;

                    let endereco = diesel::insert_into(enderecosEntrega::table)
                        .values((
                            enderecosEntrega::id.eq(Uuid::new_v4().to_string()),
                            enderecosEntrega::idPedido.eq(&id_pedido),
                            enderecosEntrega::nomeRemetente.eq(&endereco_entrega.nome_remetente),
                            enderecosEntrega::cep.eq(&endereco_entrega.cep),
                            enderecosEntrega::logradouro.eq(&endereco_entrega.logradouro),
                            enderecosEntrega::numero.eq(&endereco_entrega.numero),
                            enderecosEntrega::complemento.eq(endereco_entrega.complemento),
                            enderecosEntrega::bairro.eq(&endereco_entrega.bairro),
                            enderecosEntrega::codigoIbgeCidade.eq(&endereco_entrega.codigo_ibge_cidade),
                            enderecosEntrega::codigoIbgeUF.eq(&endereco_entrega.codigo_ibge_uf),
                            enderecosEntrega::createdAt.eq(diesel::dsl::now),
                            enderecosEntrega::updatedAt.eq(diesel::dsl::now),
                        ))
                        .get_result::<EnderecosEntrega>(conn)?;

                    let pagamento_valor_total =
                        BigDecimal::from_str(&pagamento.valor_total.to_string())
                            .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    let pagamento_valor_parcela =
                        BigDecimal::from_str(&pagamento.valor_parcela.to_string())
                            .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    let pagamento_record = diesel::insert_into(pagamentos::table)
                        .values((
                            pagamentos::id.eq(Uuid::new_v4().to_string()),
                            pagamentos::idPedido.eq(&id_pedido),
                            pagamentos::formaPagamento.eq(&pagamento.forma_pagamento),
                            pagamentos::numeroParcelas.eq(pagamento.numero_parcelas as i16),
                            pagamentos::valorTotal.eq(pagamento_valor_total),
                            pagamentos::valorParcela.eq(pagamento_valor_parcela),
                            pagamentos::createdAt.eq(diesel::dsl::now),
                            pagamentos::updatedAt.eq(diesel::dsl::now),
                        ))
                        .get_result::<Pagamento>(conn)?;

                    let mut produtos_inseridos = Vec::new();
                    for produto in produtos {
                        let produto_valor_unitario =
                            BigDecimal::from_str(&produto.valor_unitario.to_string())
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                        let produto_valor_bruto =
                            BigDecimal::from_str(&produto.valor_bruto.to_string())
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                        let produto_valor_desconto =
                            BigDecimal::from_str(&produto.valor_desconto.to_string())
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                        let produto_valor_liquido =
                            BigDecimal::from_str(&produto.valor_liquido.to_string())
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                        let produto_valor_frete =
                            BigDecimal::from_str(&produto.valor_frete.to_string())
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                        let quantidade_bd = BigDecimal::from_str(&produto.quantidade.to_string())
                            .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                        let produto_pedido = diesel::insert_into(produtosPedido::table)
                            .values((
                                produtosPedido::id.eq(Uuid::new_v4().to_string()),
                                produtosPedido::idPedido.eq(&id_pedido),
                                produtosPedido::skuProduto.eq(&produto.sku_produto),
                                produtosPedido::quantidade.eq(quantidade_bd),
                                produtosPedido::valorUnitario.eq(produto_valor_unitario),
                                produtosPedido::valorBruto.eq(produto_valor_bruto),
                                produtosPedido::valorDesconto.eq(produto_valor_desconto),
                                produtosPedido::valorLiquido.eq(produto_valor_liquido),
                                produtosPedido::valorFrete.eq(produto_valor_frete),
                                produtosPedido::createdAt.eq(diesel::dsl::now),
                                produtosPedido::updatedAt.eq(diesel::dsl::now),
                            ))
                            .get_result::<ProdutosPedido>(conn)?;

                        produtos_inseridos.push(produto_pedido);
                    }

                    let mut pedido_json = serde_json::to_value(pedido)
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    if let Value::Object(ref mut pedido_obj) = pedido_json {
                        pedido_obj.insert(
                            "enderecosEntrega".to_string(),
                            serde_json::to_value(endereco)
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?,
                        );

                        pedido_obj.insert(
                            "pagamentos".to_string(),
                            serde_json::to_value(pagamento_record)
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?,
                        );

                        pedido_obj.insert(
                            "produtos".to_string(),
                            serde_json::to_value(produtos_inseridos)
                                .map_err(|_| diesel::result::Error::RollbackTransaction)?,
                        );
                    }

                    Ok::<Value, diesel::result::Error>(pedido_json)
                })
                .map_err(ApiError::from)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }
}
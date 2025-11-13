use chrono::{Utc, Duration};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::{BigDecimal, ToPrimitive};
use crate::dal::{pedido_dal::PedidoDal, produto_dal::ProdutoDal};
use crate::utils::app_message::AppMessage;
use crate::utils::tabela_frete::MAPA_FRETE;
use crate::db::DbPool;
use crate::models::pedido::{EnderecoPayload, PagamentoPayload, PedidoPayload};
use crate::models::produto::ProdutoPayload;

pub struct PedidoService;

impl PedidoService {
    pub async fn create(pool: &DbPool, mut payload: Value) -> Result<Value, AppMessage> {
        let produtos = payload["produtos"].as_array().ok_or_else(||
            AppMessage::new("Formato inválido para produtos", 400)
        )?;

        if produtos.is_empty() {
            return Err(AppMessage::new("Nenhum produto informado", 400));
        }

        let mut sku_produtos = Vec::new();
        for produto in produtos {
            if let Some(sku) = produto["skuProduto"].as_str() {
                sku_produtos.push(sku.to_string());
            }
        }

        if let Some(obj) = payload.as_object_mut() {
            obj.insert("valorBruto".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            obj.insert("valorDesconto".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
        }

        let lst_produtos = ProdutoDal::get_by_skus(pool, sku_produtos.clone(), "").await
            .map_err(|e| AppMessage::new(&format!("Erro ao buscar produtos: {}", e), 500))?;

        let mut map_produtos = HashMap::new();
        for produto in &lst_produtos {
            map_produtos.insert(produto.sku.clone(), produto.clone());
        }

        if sku_produtos.len() != lst_produtos.len() {
            for sku in sku_produtos {
                if !map_produtos.contains_key(&sku) {
                    return Err(AppMessage::new(&format!("Produto com SKU igual a \"{}\" não existe", sku), 400));
                }
            }
        }
        let codigo_ibge_uf = match &payload["enderecoEntrega"]["codigoIbgeUF"] {
            Value::String(s) => s.parse::<i64>()
                .map_err(|_| AppMessage::new(&format!("Código IBGE UF '{}' deve ser um número válido", s), 400))?,
            Value::Number(n) => n.as_i64()
                .ok_or_else(|| AppMessage::new("Código IBGE UF inválido", 400))?,
            _ => return Err(AppMessage::new("Código IBGE UF não informado ou tipo inválido", 400))
        };

        let valor_frete = MAPA_FRETE.get(&codigo_ibge_uf)
            .ok_or_else(|| AppMessage::new(&format!("Código IBGE UF {} não encontrado na tabela de fretes", codigo_ibge_uf), 400))?;

        let mut valor_bruto_total = 0.0;
        let mut valor_desconto_total = 0.0;

        let porcentagem_desconto = payload["pagamento"]["porcentagemDesconto"].as_f64().unwrap_or(0.0);

        if let Some(produtos_array) = payload["produtos"].as_array_mut() {
            let num_produtos = produtos_array.len() as f64;

            for produto in &mut *produtos_array {
                if let (Some(sku), Some(quantidade)) = (
                    produto["skuProduto"].as_str(),
                    produto["quantidade"].as_i64(),
                ) {
                    let produto_db = map_produtos.get(sku)
                        .ok_or_else(|| AppMessage::new(&format!("Produto com SKU igual a \"{}\" não existe", sku), 400))?;

                    let estoque = produto_db.estoque.to_f64()
                        .ok_or_else(|| AppMessage::new(&format!("Estoque inválido para produto {}", sku), 500))?;

                    if estoque < quantidade as f64 {
                        return Err(AppMessage::new(&format!("Produto com SKU igual a \"{}\" está com estoque em falta", sku), 400));
                    }

                    let valor_unitario = produto_db.preco.to_f64()
                        .ok_or_else(|| AppMessage::new(&format!("Preço inválido para produto {}", sku), 500))?;

                    let valor_bruto = valor_unitario * quantidade as f64;

                    let valor_desconto = if porcentagem_desconto > 0.0 {
                        (valor_bruto * porcentagem_desconto * 100.0).round() / 100.0
                    } else {
                        0.0
                    };

                    let valor_liquido = valor_bruto - valor_desconto;
                    let valor_frete_produto = ((valor_frete / num_produtos) * 100.0).round() / 100.0;

                    if let Some(obj) = produto.as_object_mut() {
                        obj.insert("valorUnitario".to_string(), Value::Number(serde_json::Number::from_f64(valor_unitario).unwrap()));
                        obj.insert("valorBruto".to_string(), Value::Number(serde_json::Number::from_f64(valor_bruto).unwrap()));
                        obj.insert("valorDesconto".to_string(), Value::Number(serde_json::Number::from_f64(valor_desconto).unwrap()));
                        obj.insert("valorLiquido".to_string(), Value::Number(serde_json::Number::from_f64(valor_liquido).unwrap()));
                        obj.insert("valorFrete".to_string(), Value::Number(serde_json::Number::from_f64(valor_frete_produto).unwrap()));
                    }

                    valor_bruto_total += valor_bruto;
                    valor_desconto_total += valor_desconto;
                }
            }
        }

        let data_atual = Utc::now().naive_utc();
        let data_entrega = data_atual + Duration::days(3);
        let data_entrega_iso = data_entrega.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();

        if let Some(obj) = payload.as_object_mut() {
            obj.insert("dataEntrega".to_string(), Value::String(data_entrega_iso));
            obj.insert("valorBruto".to_string(), Value::Number(serde_json::Number::from_f64(valor_bruto_total).unwrap()));
            obj.insert("valorDesconto".to_string(), Value::Number(serde_json::Number::from_f64(valor_desconto_total).unwrap()));
            obj.insert("valorFrete".to_string(), Value::Number(serde_json::Number::from_f64(*valor_frete).unwrap()));

            let valor_liquido_total = valor_bruto_total - valor_desconto_total + valor_frete;
            obj.insert("valorLiquido".to_string(), Value::Number(serde_json::Number::from_f64(valor_liquido_total).unwrap()));
            obj.insert("status".to_string(), Value::String("P".to_string()));

            if let Some(pagamento) = obj.get_mut("pagamento").and_then(|p| p.as_object_mut()) {
                pagamento.remove("porcentagemDesconto");
                pagamento.insert("valorTotal".to_string(), Value::Number(serde_json::Number::from_f64(valor_liquido_total).unwrap()));

                if let Some(num_parcelas) = pagamento.get("numeroParcelas").and_then(|n| n.as_i64()) {
                    let valor_parcela = ((valor_liquido_total / num_parcelas as f64) * 100.0).round() / 100.0;
                    pagamento.insert("valorParcela".to_string(), Value::Number(serde_json::Number::from_f64(valor_parcela).unwrap()));
                }
            }

            if let Some(endereco) = obj.get_mut("enderecoEntrega").and_then(|e| e.as_object_mut()) {
                if let Some(uf_value) = endereco.get("codigoIbgeUF") {
                    let uf_string = match uf_value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => uf_value.to_string()
                    };
                    endereco.insert("codigoIbgeUF".to_string(), Value::String(uf_string));
                }

                if let Some(cidade_value) = endereco.get("codigoIbgeCidade") {
                    let cidade_string = match cidade_value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => cidade_value.to_string()
                    };
                    endereco.insert("codigoIbgeCidade".to_string(), Value::String(cidade_string));
                }
            }
        }

        payload.get("idCliente")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| AppMessage::new("idCliente não encontrado", 400))?;

        payload.get("valorBruto")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| AppMessage::new("valorBruto não encontrado", 400))?;

        payload.get("valorDesconto")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| AppMessage::new("valorDesconto não encontrado", 400))?;

        payload.get("valorFrete")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| AppMessage::new("valorFrete não encontrado", 400))?;

        payload.get("valorLiquido")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| AppMessage::new("valorLiquido não encontrado", 400))?;

        payload.get("dataEntrega")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppMessage::new("dataEntrega não encontrado", 400))?;

        if let Some(endereco) = payload.get("enderecoEntrega") {
            if let Err(e) = serde_json::from_value::<EnderecoPayload>(endereco.clone()) {
                return Err(AppMessage::new(&format!("Erro no endereço: {}", e), 400));
            }
        }

        if let Some(pagamento) = payload.get("pagamento") {
            if let Err(e) = serde_json::from_value::<PagamentoPayload>(pagamento.clone()) {
                return Err(AppMessage::new(&format!("Erro no pagamento: {}", e), 400));
            }
        }

        if let Some(produtos) = payload.get("produtos") {
            if let Err(e) = serde_json::from_value::<Vec<ProdutoPayload>>(produtos.clone()) {
                return Err(AppMessage::new(&format!("Erro nos produtos: {}", e), 400));
            }
        }

        let payload_json_string = match serde_json::to_string(&payload) {
            Ok(s) => {
                s
            },
            Err(e) => {
                return Err(AppMessage::new(&format!("Erro ao serializar payload: {}", e), 400));
            }
        };

        let pedido_payload: PedidoPayload = match serde_json::from_str(&payload_json_string) {
            Ok(p) => {
                p
            },
            Err(e) => {
                match serde_json::from_value::<PedidoPayload>(payload.clone()) {
                    Ok(_) => {
                        return match serde_json::from_value(payload.clone()) {
                            Ok(pedido_payload) => {

                                let pedido = PedidoDal::create(pool, pedido_payload).await
                                    .map_err(|e| AppMessage::new(&format!("Erro ao criar pedido: {}", e), 500))?;

                                if let Some(produtos) = pedido.get("produtos").and_then(|p| p.as_array()) {
                                    for produto in produtos {
                                        if let (Some(sku), Some(quantidade)) = (
                                            produto.get("skuProduto").and_then(|s| s.as_str()),
                                            produto.get("quantidade").and_then(|q| q.as_i64())
                                        ) {
                                            ProdutoDal::update_estoque(
                                                pool,
                                                &sku.to_string(),
                                                BigDecimal::from_str(&(-quantidade).to_string()).unwrap()
                                            ).await
                                                .map_err(|e| AppMessage::new(&format!("Erro ao atualizar estoque: {}", e), 500))?;

                                            ProdutoDal::update_vendas(
                                                pool,
                                                &sku.to_string(),
                                                BigDecimal::from_str(&quantidade.to_string()).unwrap()
                                            ).await
                                                .map_err(|e| AppMessage::new(&format!("Erro ao atualizar vendas: {}", e), 500))?;
                                        }
                                    }
                                }

                                Ok(pedido)
                            },
                            Err(e3) => Err(AppMessage::new(&format!("Erro ao converter payload: {}", e3), 400))
                        };
                    },
                    Err(e2) => println!("from_value falha: {}", e2),
                }

                return Err(AppMessage::new(&format!("Erro ao converter payload: {}", e), 400));
            }
        };

        let pedido = PedidoDal::create(pool, pedido_payload).await
            .map_err(|e| AppMessage::new(&format!("Erro ao criar pedido: {}", e), 500))?;

        if let Some(produtos) = pedido.get("produtos").and_then(|p| p.as_array()) {
            for produto in produtos {
                if let (Some(sku), Some(quantidade)) = (
                    produto.get("skuProduto").and_then(|s| s.as_str()),
                    produto.get("quantidade").and_then(|q| q.as_i64())
                ) {
                    ProdutoDal::update_estoque(
                        pool,
                        &sku.to_string(),
                        BigDecimal::from_str(&(-quantidade).to_string()).unwrap()
                    ).await
                        .map_err(|e| AppMessage::new(&format!("Erro ao atualizar estoque: {}", e), 500))?;

                    ProdutoDal::update_vendas(
                        pool,
                        &sku.to_string(),
                        BigDecimal::from_str(&quantidade.to_string()).unwrap()
                    ).await
                        .map_err(|e| AppMessage::new(&format!("Erro ao atualizar vendas: {}", e), 500))?;
                }
            }
        }

        Ok(pedido)
    }
}
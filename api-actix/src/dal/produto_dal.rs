use std::str::FromStr;
use bigdecimal::BigDecimal;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Double;
use crate::schema::{categorias, produtos};
use crate::db::DbPool;
use crate::models::categoria::Categoria;
use crate::models::produto::Produto;
use crate::utils::app_message::{ApiError, AppMessage};

pub struct ProdutoDal;

impl ProdutoDal {
    pub async fn get_all(pool: &DbPool, campos: &str, page: u32, mut page_size: u32) -> Result<Vec<Produto>, ApiError> {
        let pool_clone = pool.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            if page_size > 100 {
                page_size = 100;
            }
            let offset = (page - 1) * page_size;

            if campos_owned.trim().is_empty() || campos_owned == "null" {
                return produtos::table
                    .select(Produto::as_select())
                    .limit(page_size as i64)
                    .offset(offset as i64)
                    .load(&mut connection)
                    .map_err(ApiError::from);
            }

            let query = format!(
                "SELECT {} FROM produtos LIMIT {} OFFSET {}",
                campos_owned,
                page_size,
                offset
            );

            sql_query(query)
                .load::<Produto>(&mut connection)
                .map_err(ApiError::from)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn get_random(pool: &DbPool, campos: &str, limite: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        let pool_clone = pool.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            let produtos_with_categorias = produtos::table
                .inner_join(categorias::table.on(produtos::idCategoria.eq(categorias::id)))
                .select((Produto::as_select(), Categoria::as_select()))
                .order_by(sql::<Double>("RANDOM()"))
                .limit(limite as i64)
                .load::<(Produto, Categoria)>(&mut connection)
                .map_err(ApiError::from)?;

            Self::process_produtos_with_categorias(produtos_with_categorias, &campos_owned)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn get_by_skus(pool: &DbPool, skus: Vec<String>, campos: &str) -> Result<Vec<Produto>, ApiError> {
        let pool_clone = pool.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            if campos_owned.trim().is_empty() || campos_owned == "null" {
                return produtos::table
                    .select(Produto::as_select())
                    .filter(produtos::sku.eq_any(skus))
                    .load(&mut connection)
                    .map_err(ApiError::from);
            }

            let skus_sanitized: Vec<String> = skus.iter()
                .map(|s| s.replace('\'', "''"))
                .collect();

            let skus_list = skus_sanitized
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ");

            let query = format!(
                "SELECT {} FROM produtos WHERE \"sku\" IN ({}) ORDER BY RANDOM()",
                campos_owned,
                skus_list
            );

            sql_query(query)
                .load::<Produto>(&mut connection)
                .map_err(ApiError::from)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn get_by_id_categoria(pool: &DbPool, id_categoria: &String, campos: &str, page: u32, mut page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        let pool_clone = pool.clone();
        let id_categoria_owned = id_categoria.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            if page_size > 100 {
                page_size = 100;
            }
            let offset = (page - 1) * page_size;

            let produtos_with_categorias = produtos::table
                .inner_join(categorias::table.on(produtos::idCategoria.eq(categorias::id)))
                .select((Produto::as_select(), Categoria::as_select()))
                .filter(produtos::idCategoria.eq(&id_categoria_owned))
                .limit(page_size as i64)
                .offset(offset as i64)
                .load::<(Produto, Categoria)>(&mut connection)
                .map_err(|e| {
                    log::error!("Database error when fetching by category: {:?}", e);
                    ApiError::from(e)
                })?;

            Self::process_produtos_with_categorias(produtos_with_categorias, &campos_owned)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn get_by_sku(pool: &DbPool, sku: &String, campos: &str) -> Result<serde_json::Value, ApiError> {
        let pool_clone = pool.clone();
        let sku_owned = sku.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            let produto_with_categoria = produtos::table
                .inner_join(categorias::table.on(produtos::idCategoria.eq(categorias::id)))
                .select((Produto::as_select(), Categoria::as_select()))
                .filter(produtos::sku.eq(&sku_owned))
                .first::<(Produto, Categoria)>(&mut connection)
                .map_err(|e| {
                    match e {
                        diesel::NotFound => ApiError::from(AppMessage::new("Produto não encontrado", 404)),
                        _ => ApiError::from(e)
                    }
                })?;

            let (produto, categoria) = produto_with_categoria;

            let mut produto_json = serde_json::to_value(produto)
                .map_err(|e| ApiError::from(AppMessage::new(&format!("JSON serialization error: {}", e), 500)))?;

            if let serde_json::Value::Object(ref mut produto_obj) = produto_json {
                Self::convert_numeric_fields(produto_obj);

                let mut categoria_obj = serde_json::Map::new();
                categoria_obj.insert("id".to_string(), serde_json::Value::String(categoria.id.to_string()));
                categoria_obj.insert("nome".to_string(), serde_json::Value::String(categoria.nome));
                produto_obj.insert("categoria".to_string(), serde_json::Value::Object(categoria_obj));
            }

            if campos_owned.trim().is_empty() || campos_owned == "null" {
                return Ok(produto_json);
            }

            let campos_vec: Vec<&str> = campos_owned.split(',').map(|c| c.trim()).collect();

            if let serde_json::Value::Object(produto_obj) = produto_json {
                let mut filtered_produto = serde_json::Map::new();

                for campo in &campos_vec {
                    let campo_str = match *campo {
                        "id_categoria" => "idCategoria".to_string(),
                        "created_at" => "createdAt".to_string(),
                        "updated_at" => "updatedAt".to_string(),
                        _ => campo.to_string(),
                    };

                    if let Some(value) = produto_obj.get(&campo_str) {
                        filtered_produto.insert(campo.to_string(), value.clone());
                    }
                }

                if campos_vec.iter().any(|campo| {
                    *campo == "id_categoria" || *campo == "idCategoria"
                }) {
                    let categoria_key = "categoria".to_string();
                    if let Some(categoria_value) = produto_obj.get(&categoria_key) {
                        filtered_produto.insert(categoria_key, categoria_value.clone());
                    }
                }

                return Ok(serde_json::Value::Object(filtered_produto));
            }

            Ok(produto_json)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn update_estoque(pool: &DbPool, sku: &String, quantidade: BigDecimal) -> Result<(), ApiError> {
        let pool_clone = pool.clone();
        let sku_owned = sku.clone();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            let produto = produtos::table
                .filter(produtos::sku.eq(&sku_owned))
                .select(produtos::estoque)
                .first::<BigDecimal>(&mut connection)
                .optional()
                .map_err(ApiError::from)?;

            if produto.is_none() {
                return Err(ApiError::from(AppMessage::new("Produto não encontrado", 404)));
            }

            let query = format!(
                "UPDATE \"produtos\" SET \"estoque\"=\"estoque\" + ({}) WHERE \"sku\" = '{}'",
                quantidade,
                sku_owned.replace('\'', "''")
            );

            sql_query(query)
                .execute(&mut connection)
                .map(|_| ())
                .map_err(ApiError::from)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn get_all_ofertas(pool: &DbPool, campos: &str, page: u32, mut page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        let pool_clone = pool.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            if page_size > 100 {
                page_size = 100;
            }
            let offset = (page - 1) * page_size;

            let produtos_with_categorias = produtos::table
                .inner_join(categorias::table.on(produtos::idCategoria.eq(categorias::id)))
                .select((Produto::as_select(), Categoria::as_select()))
                .filter(produtos::pctoferta.gt(BigDecimal::from_str("0").unwrap()))
                .order_by(produtos::pctoferta.desc())
                .limit(page_size as i64)
                .offset(offset as i64)
                .load::<(Produto, Categoria)>(&mut connection)
                .map_err(ApiError::from)?;

            Self::process_produtos_with_categorias(produtos_with_categorias, &campos_owned)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn get_all_destaques(pool: &DbPool, campos: &str, page: u32, mut page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        let pool_clone = pool.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            if page_size > 100 {
                page_size = 100;
            }
            let offset = (page - 1) * page_size;

            let produtos_with_categorias = produtos::table
                .inner_join(categorias::table.on(produtos::idCategoria.eq(categorias::id)))
                .select((Produto::as_select(), Categoria::as_select()))
                .order_by(produtos::pctoferta.desc())
                .limit(page_size as i64)
                .offset(offset as i64)
                .load::<(Produto, Categoria)>(&mut connection)
                .map_err(ApiError::from)?;

            Self::process_produtos_with_categorias(produtos_with_categorias, &campos_owned)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    pub async fn update_vendas(pool: &DbPool, sku: &String, quantidade: BigDecimal) -> Result<(), ApiError> {
        let pool_clone = pool.clone();
        let sku_owned = sku.clone();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            let produto = produtos::table
                .filter(produtos::sku.eq(&sku_owned))
                .select(produtos::estoque)
                .first::<BigDecimal>(&mut connection)
                .optional()
                .map_err(ApiError::from)?;

            if produto.is_none() {
                return Err(ApiError::from(AppMessage::new("Produto não encontrado", 404)));
            }

            let query = format!(
                "UPDATE \"produtos\" SET \"qtdvendas\"=\"qtdvendas\" + ({}) WHERE \"sku\" = '{}'",
                quantidade,
                sku_owned.replace('\'', "''")
            );

            sql_query(query)
                .execute(&mut connection)
                .map(|_| ())
                .map_err(ApiError::from)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

    fn process_produtos_with_categorias(
        produtos_with_categorias: Vec<(Produto, Categoria)>,
        campos: &str
    ) -> Result<Vec<serde_json::Value>, ApiError> {
        let mut produtos_array = Vec::new();
        for (produto, categoria) in produtos_with_categorias {
            let mut produto_json = serde_json::to_value(produto)
                .map_err(|e| ApiError::from(AppMessage::new(&format!("JSON serialization error: {}", e), 500)))?;

            if let serde_json::Value::Object(ref mut produto_obj) = produto_json {
                Self::convert_numeric_fields(produto_obj);

                let mut categoria_obj = serde_json::Map::new();
                categoria_obj.insert("id".to_string(), serde_json::Value::String(categoria.id.to_string()));
                categoria_obj.insert("nome".to_string(), serde_json::Value::String(categoria.nome));
                produto_obj.insert("categoria".to_string(), serde_json::Value::Object(categoria_obj));
            }

            produtos_array.push(produto_json);
        }

        if campos.trim().is_empty() || campos == "null" {
            return Ok(produtos_array);
        }

        let campos_vec: Vec<&str> = campos.split(',').map(|c| c.trim()).collect();

        let mut filtered_result = Vec::new();
        for produto in produtos_array {
            if let serde_json::Value::Object(produto_obj) = produto {
                let mut filtered_produto = serde_json::Map::new();

                for campo in &campos_vec {
                    let campo_str = match *campo {
                        "id_categoria" => "idCategoria".to_string(),
                        "created_at" => "createdAt".to_string(),
                        "updated_at" => "updatedAt".to_string(),
                        _ => campo.to_string(),
                    };

                    if let Some(value) = produto_obj.get(&campo_str) {
                        filtered_produto.insert(campo.to_string(), value.clone());
                    }
                }

                if campos_vec.iter().any(|campo| {
                    *campo == "id_categoria" || *campo == "idCategoria"
                }) {
                    let categoria_key = "categoria".to_string();
                    if let Some(categoria_value) = produto_obj.get(&categoria_key) {
                        filtered_produto.insert(categoria_key, categoria_value.clone());
                    }
                }

                filtered_result.push(serde_json::Value::Object(filtered_produto));
            }
        }

        Ok(filtered_result)
    }

    fn convert_numeric_fields(produto_obj: &mut serde_json::Map<String, serde_json::Value>) {
        if let Some(pct_oferta) = produto_obj.get("pctoferta") {
            if let serde_json::Value::String(pct_str) = pct_oferta {
                if let Ok(pct_value) = pct_str.parse::<f64>() {
                    produto_obj.insert("pctoferta".to_string(),
                                       serde_json::Value::Number(serde_json::Number::from_f64(pct_value)
                                           .unwrap_or(serde_json::Number::from(0))));
                }
            }
        }

        if let Some(preco) = produto_obj.get("preco") {
            if let serde_json::Value::String(preco_str) = preco {
                if let Ok(preco_value) = preco_str.parse::<f64>() {
                    produto_obj.insert("preco".to_string(),
                                       serde_json::Value::Number(serde_json::Number::from_f64(preco_value)
                                           .unwrap_or(serde_json::Number::from(0))));
                }
            }
        }
    }

    pub async fn get_by_nome(pool: &DbPool, nome: &String, campos: &str, page: u32, mut page_size: u32) -> Result<Vec<serde_json::Value>, ApiError> {
        let pool_clone = pool.clone();
        let nome_owned = nome.clone();
        let campos_owned = campos.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            if page_size > 100 {
                page_size = 100;
            }
            let offset = (page - 1) * page_size;

            let produtos_with_categorias = produtos::table
                .inner_join(categorias::table.on(produtos::idCategoria.eq(categorias::id)))
                .select((Produto::as_select(), Categoria::as_select()))
                .filter(produtos::nome.ilike(format!("%{}%", nome_owned)))
                .limit(page_size as i64)
                .offset(offset as i64)
                .load::<(Produto, Categoria)>(&mut connection)
                .map_err(|e| {
                    log::error!("Database error when searching by name contains: {:?}", e);
                    ApiError::from(e)
                })?;

            Self::process_produtos_with_categorias(produtos_with_categorias, &campos_owned)
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }
}
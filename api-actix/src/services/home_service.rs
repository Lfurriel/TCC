use crate::utils::app_message::ApiError;
use crate::db::DbPool;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::dal::categoria_dal::CategoriaDal;
use crate::dal::produto_dal::ProdutoDal;
use crate::models::categoria::CategoriaResumo;

pub struct HomeService;

impl HomeService {
    pub async fn get_home_amazon(pool: &DbPool) -> Result<HomeAmazon, ApiError> {
        let mut home = HomeAmazon::default();

        let (ofertas_result, categorias_result) = tokio::join!(
            ProdutoDal::get_all_ofertas(pool, "sku, foto, pctoferta, idCategoria", 1, 4),
            CategoriaDal::get_all(pool)
        );

        home.ofertas = ofertas_result?;
        let all_categorias = categorias_result?;

        if all_categorias.len() > 7 {
            let mut rng = rand::rng();
            let mut shuffled_categories = all_categorias.clone();
            shuffled_categories.shuffle(&mut rng);
            home.categorias = shuffled_categories.into_iter().take(7).collect();
        } else {
            home.categorias = all_categorias.clone();
        }

        let mut produtos: Vec<serde_json::Value> = Vec::new();
        for categoria in &home.categorias {
            let produtos_categoria: Vec<serde_json::Value> = ProdutoDal::get_by_id_categoria(
                pool,
                &categoria.id,
                "sku, nome, foto, preco, idCategoria",
                1,
                20
            ).await?
                .into_iter()
                .map(|item| serde_json::to_value(item).unwrap_or_default())
                .collect();

            if !produtos_categoria.is_empty() {
                produtos.extend(produtos_categoria);
            }
        }

        home.categorias = all_categorias;
        home.produtos = produtos;

        Ok(home)
    }

    pub async fn get_home_shopee(pool: &DbPool) -> Result<HomeShopee, ApiError> {
        let mut home = HomeShopee::default();

        let (ofertas_result, categorias_result, produtos_result, destaques_result) = tokio::join!(
            ProdutoDal::get_all_ofertas(
                pool,
                "sku, foto, pctoferta, preco, idCategoria",
                1,
                15
            ),
            CategoriaDal::get_all(pool),
            ProdutoDal::get_random(
                pool,
                "sku, nome, foto, pctoferta, preco, idCategoria",
                36
            ),
            ProdutoDal::get_all_destaques(
                pool,
                "sku, nome, pctoferta, idCategoria, qtdvendas",
                1,
                15
            )
        );

        home.ofertas = ofertas_result?;
        home.categorias = categorias_result?;
        home.produtos = produtos_result?;
        home.destaques = Some(destaques_result?);

        Ok(home)
    }
}

#[derive(Deserialize, Serialize)]
pub struct HomeAmazon {
    pub ofertas: Vec<Value>,
    pub categorias: Vec<CategoriaResumo>,
    pub produtos: Vec<Value>,
}

impl Default for HomeAmazon {
    fn default() -> Self {
        Self {
            ofertas: Vec::new(),
            categorias: Vec::new(),
            produtos: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HomeShopee {
    pub ofertas: Vec<Value>,
    pub categorias: Vec<CategoriaResumo>,
    pub produtos: Vec<Value>,
    pub destaques: Option<Vec<Value>>,
}

impl Default for HomeShopee {
    fn default() -> Self {
        Self {
            ofertas: Vec::new(),
            categorias: Vec::new(),
            produtos: Vec::new(),
            destaques: None,
        }
    }
}
use diesel::prelude::*;
use crate::schema::categorias;
use crate::db::DbPool;
use crate::models::categoria::CategoriaResumo;
use crate::utils::app_message::{ApiError, AppMessage};

pub struct CategoriaDal;

impl CategoriaDal {
    pub async fn get_all(pool: &DbPool) -> Result<Vec<CategoriaResumo>, ApiError> {
        let pool_clone = pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| ApiError::from(AppMessage::new(&format!("Database connection error: {}", e), 500)))?;

            categorias::table
                .select((categorias::id, categorias::nome))
                .load::<CategoriaResumo>(&mut connection)
                .map_err(|e| ApiError::from(e))
        }).await
            .map_err(|e| ApiError::from(AppMessage::new(&format!("Task error: {}", e), 500)))?
    }

}
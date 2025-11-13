use crate::dal::categoria_dal::CategoriaDal;
use crate::utils::app_message::ApiError;
use crate::db::DbPool;
use crate::models::categoria::CategoriaResumo;

pub struct CategoriaService;

impl CategoriaService {
    pub async fn get_all(pool: &DbPool) -> Result<Vec<CategoriaResumo>, ApiError> {
        CategoriaDal::get_all(pool).await
    }
}
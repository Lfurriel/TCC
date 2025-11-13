use diesel::{Queryable, Identifiable, Selectable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::categorias;

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable, Deserialize, Clone)]
#[diesel(table_name = categorias)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Categoria {
    pub id: String,
    pub nome: String,
    #[diesel(column_name = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct CategoriaResumo {
    pub id: String,
    pub nome: String,
}
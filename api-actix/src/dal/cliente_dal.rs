use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::cliente::{Cliente, NewCliente};
use crate::utils::app_message::AppMessage;

pub struct ClienteDal;

impl ClienteDal {
    pub async fn create(pool: &DbPool, payload: NewCliente) -> Result<Cliente, AppMessage> {
        use crate::schema::clientes::dsl::*;
        use diesel::result::{Error as DieselError, DatabaseErrorKind};

        let pool_clone = pool.clone();
        let payload_owned = payload;

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| AppMessage::new(&format!("Database connection error: {}", e), 500))?;

            match diesel::insert_into(clientes)
                .values(&payload_owned)
                .get_result::<Cliente>(&mut connection)
            {
                Ok(cliente) => Ok(cliente),
                Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
                    let error_message = if let Some(constraint) = info.constraint_name() {
                        match constraint {
                            "clientes_email_key" | "idx_clientes_email" => "Cliente com email já cadastrado",
                            "clientes_cpf_key" | "idx_clientes_cpf" => "Cliente com CPF já cadastrado",
                            "clientes_cnpj_key" | "idx_clientes_cnpj" => "Cliente com CNPJ já cadastrado",
                            "clientes_telefone_key" | "idx_clientes_telefone" => "Cliente com telefone já cadastrado",
                            _ => "Cliente já cadastrado com esses dados"
                        }
                    } else {
                        let details = info.message();
                        if details.contains("email") {
                            "Cliente com email já cadastrado"
                        } else if details.contains("cpf") {
                            "Cliente com CPF já cadastrado"
                        } else if details.contains("cnpj") {
                            "Cliente com CNPJ já cadastrado"
                        } else if details.contains("telefone") {
                            "Cliente com telefone já cadastrado"
                        } else {
                            "Cliente já cadastrado"
                        }
                    };

                    Err(AppMessage::new(error_message, 400))
                },
                Err(e) => Err(AppMessage::new(&format!("Erro ao criar cliente: {}", e), 500))
            }
        }).await
            .map_err(|e| AppMessage::new(&format!("Task error: {}", e), 500))?
    }

    pub async fn get_by_email(pool: &DbPool, email_param: &str) -> Result<Cliente, AppMessage> {
        use crate::schema::clientes::dsl::*;

        let pool_clone = pool.clone();
        let email_owned = email_param.to_string();

        tokio::task::spawn_blocking(move || {
            let mut connection = pool_clone.get()
                .map_err(|e| AppMessage::new(&format!("Database connection error: {}", e), 500))?;

            let cliente = clientes
                .filter(email.eq(&email_owned))
                .first::<Cliente>(&mut connection)
                .optional()
                .map_err(|e| AppMessage::new(&format!("Erro ao buscar cliente: {}", e), 500))?;

            match cliente {
                Some(c) => Ok(c),
                None => Err(AppMessage::new("Cliente não encontrado", 404)),
            }
        }).await
            .map_err(|e| AppMessage::new(&format!("Task error: {}", e), 500))?
    }
}
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::schema::clientes;
use crate::validations::cliente_validations::{validate_cpf, validate_cnpj, validate_data_nascimento, validate_ie, validate_senha_forte, validate_telefone, validate_sexo, validate_tipo_pessoa};

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Cliente {
    pub id: String,
    #[diesel(column_name = "tipoPessoa")]
    #[serde(rename = "tipoPessoa")]
    pub tipo_pessoa: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub nome: String,
    pub ie: Option<String>,
    #[diesel(column_name = "razaoSocial")]
    #[serde(rename = "razaoSocial")]
    pub razao_social: Option<String>,
    #[diesel(column_name = "dataNascimento")]
    #[serde(rename = "dataNascimento")]
    pub data_nascimento: NaiveDate,
    pub sexo: String,
    pub email: String,
    pub telefone: String,
    pub senha: String,
    #[diesel(column_name = "createdAt")]
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClienteResponse {
    pub id: String,
    pub tipo_pessoa: String,
    pub nome: String,
    pub ie: Option<String>,
    pub razao_social: Option<String>,
    pub data_nascimento: chrono::NaiveDate,
    pub sexo: String,
    pub email: String,
    pub telefone: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<Cliente> for ClienteResponse {
    fn from(cliente: Cliente) -> Self {
        ClienteResponse {
            id: cliente.id,
            tipo_pessoa: cliente.tipo_pessoa,
            nome: cliente.nome,
            ie: cliente.ie,
            razao_social: cliente.razao_social,
            data_nascimento: cliente.data_nascimento,
            sexo: cliente.sexo,
            email: cliente.email,
            telefone: cliente.telefone,
            created_at: Some(cliente.created_at),
            updated_at: Some(cliente.updated_at),
            // Don't include senha, cpf, cnpj - sensitive data
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::clientes)]
pub struct NewCliente {
    pub id: String,
    #[diesel(column_name = "tipoPessoa")]
    #[serde(rename = "tipoPessoa")]
    pub tipo_pessoa: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub nome: String,
    pub ie: Option<String>,
    #[diesel(column_name = "razaoSocial")]
    #[serde(rename = "razaoSocial")]
    pub razao_social: Option<String>,
    #[diesel(column_name = "dataNascimento")]
    #[serde(rename = "dataNascimento")]
    pub data_nascimento: chrono::NaiveDate,
    pub sexo: String,
    pub email: String,
    pub telefone: String,
    pub senha: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateClientePayload {
    #[serde(rename = "tipoPessoa")]
    #[validate(custom = "validate_tipo_pessoa")]
    pub tipo_pessoa: String,

    #[validate(custom = "validate_cpf")]
    pub cpf: Option<String>,

    #[validate(custom = "validate_cnpj")]
    pub cnpj: Option<String>,

    #[validate(length(min = 1, max = 60, message = "Nome deve ter entre 1 e 60 caracteres"))]
    pub nome: String,

    #[validate(custom = "validate_ie")]
    pub ie: Option<String>,

    #[serde(rename = "razaoSocial")]
    #[validate(length(max = 60, message = "Razão social deve ter no máximo 60 caracteres"))]
    pub razao_social: Option<String>,

    #[serde(rename = "dataNascimento")]
    #[validate(custom = "validate_data_nascimento")]
    pub data_nascimento: String,

    #[validate(custom = "validate_sexo")]
    pub sexo: String,

    #[validate(email(message = "Formato de email inválido"))]
    pub email: String,

    #[validate(custom = "validate_telefone")]
    pub telefone: String,

    #[validate(custom = "validate_senha_forte")]
    pub senha: String,

    #[serde(rename = "confirmarSenha")]
    pub confirmar_senha: String,
}

impl From<CreateClientePayload> for NewCliente {
    fn from(payload: CreateClientePayload) -> Self {
        // Parse date string to NaiveDate
        let data_nascimento = chrono::NaiveDate::parse_from_str(&payload.data_nascimento, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        NewCliente {
            id: Uuid::new_v4().to_string(),
            tipo_pessoa: payload.tipo_pessoa,
            cpf: payload.cpf,
            cnpj: payload.cnpj,
            nome: payload.nome,
            ie: payload.ie,
            razao_social: payload.razao_social,
            data_nascimento,
            sexo: payload.sexo,
            email: payload.email.to_lowercase(),
            telefone: payload.telefone,
            senha: payload.senha,
        }
    }
}
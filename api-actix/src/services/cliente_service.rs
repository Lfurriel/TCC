use crate::dal::cliente_dal::ClienteDal;
use crate::utils::app_message::{AppMessage};
use crate::utils::hash_password::{hash_password, compare_passwords};
use crate::configs::auth::AuthConfig;
use crate::db::DbPool;
use jsonwebtoken::{encode, Header, EncodingKey};
pub struct ClienteService;
use validator::{Validate};
use serde::{Serialize, Deserialize};
use crate::models::cliente::{ClienteResponse, CreateClientePayload, NewCliente};
use crate::models::login::{Claims, LoginResponse};
use crate::validations::cliente_validations::{validate_cnpj, validate_cpf, validate_tipo_pessoa_consistency};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Formato de email inválido"))]
    pub email: String,

    #[validate(length(min = 1, message = "Senha é obrigatória"))]
    pub senha: String,
}

impl ClienteService {
    pub async fn create(pool: &DbPool, mut payload: CreateClientePayload) -> Result<LoginResponse, AppMessage> {
        payload.email = payload.email.to_lowercase();

        if payload.senha != payload.confirmar_senha {
            return Err(AppMessage::new("Senhas não coincidem", 400));
        }

        if let Err(errors) = payload.validate() {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(_, errors)| errors.iter().map(|e| e.message.as_ref().unwrap_or(&"Erro de validação".into()).to_string()))
                .collect();
            return Err(AppMessage::new(&error_messages.join(", "), 400));
        }

        let validation_handle = {
            let payload_clone = payload.clone();
            tokio::task::spawn_blocking(move || -> Result<(), AppMessage> {
                if let Err(error) = validate_tipo_pessoa_consistency(&payload_clone) {
                    return Err(AppMessage::new(&error.message.unwrap_or("Dados inconsistentes".into()), 400));
                }

                match payload_clone.tipo_pessoa.as_str() {
                    "PF" => {
                        if let Some(ref cpf) = payload_clone.cpf {
                            validate_cpf(cpf).map_err(|_| AppMessage::new("CPF inválido", 400))?;
                        }
                    },
                    "PJ" => {
                        if let Some(ref cnpj) = payload_clone.cnpj {
                            validate_cnpj(cnpj).map_err(|_| AppMessage::new("CNPJ inválido", 400))?;
                        }
                    },
                    _ => return Err(AppMessage::new("Tipo de pessoa inválido", 400))
                }

                Ok(())
            })
        };

        let hash_handle = {
            let senha = payload.senha.clone();
            tokio::task::spawn_blocking(move || hash_password(&senha))
        };

        let (validation_result, hash_result) = tokio::try_join!(
            validation_handle,
            hash_handle
        ).map_err(|_| AppMessage::new("Erro interno", 500))?;

        validation_result?;
        payload.senha = hash_result.map_err(|_| AppMessage::new("Erro ao processar senha", 500))?;

        let new_cliente = NewCliente::from(payload.clone());

        let cliente_criado = ClienteDal::create(pool, new_cliente).await?;

        Self::generate_login_response_from_cliente(cliente_criado)
    }

    pub async fn login(pool: &DbPool, payload: LoginPayload) -> Result<LoginResponse, AppMessage> {
        if let Err(errors) = payload.validate() {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(_, errors)| errors.iter().map(|e| e.message.as_ref().unwrap_or(&"Erro de validação".into()).to_string()))
                .collect();
            return Err(AppMessage::new(&error_messages.join(", "), 400));
        }

        Self::execute_login(pool, payload).await
    }

    fn generate_login_response_from_cliente(cliente: crate::models::cliente::Cliente) -> Result<LoginResponse, AppMessage> {
        let cliente_response = ClienteResponse::from(cliente);
        let auth_config = AuthConfig::new();

        let claims = Claims {
            cliente: serde_json::to_string(&cliente_response)
                .map_err(|_| AppMessage::new("Erro ao serializar dados do cliente", 500))?,
            exp: (chrono::Utc::now() + chrono::Duration::seconds(auth_config.expires_in)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(auth_config.secret.as_ref())
        ).map_err(|_| AppMessage::new("Erro ao gerar token", 500))?;

        Ok(LoginResponse {
            cliente: cliente_response,
            token,
        })
    }

    pub async fn execute_login(pool: &DbPool, payload: LoginPayload) -> Result<LoginResponse, AppMessage> {
        let email_lower = payload.email.to_lowercase();
        let cliente = ClienteDal::get_by_email(pool, &email_lower).await?;

        let senha_valida = tokio::task::spawn_blocking({
            let payload_senha = payload.senha.clone();
            let cliente_senha = cliente.senha.clone();
            move || compare_passwords(&payload_senha, &cliente_senha)
        }).await
            .map_err(|_| AppMessage::new("Erro ao verificar senha", 500))?
            .map_err(|_| AppMessage::new("Erro ao verificar senha", 500))?;

        if !senha_valida {
            return Err(AppMessage::new("Senha incorreta", 401));
        }

        Self::generate_login_response_from_cliente(cliente)
    }
}
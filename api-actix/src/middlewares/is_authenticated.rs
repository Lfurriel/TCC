use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use crate::utils::app_message::{AppMessage};
use crate::configs::auth::AuthConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClienteAuth {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub cliente: String,
    pub exp: usize,
}

// Custom error type for authentication
#[derive(Debug)]
pub struct AuthError(pub AppMessage);

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(&self.0)
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authentication error: {}", self.0.message)
    }
}

// Middleware factory
pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extrai o header Authorization
            let auth_header = req.headers().get("authorization");

            if auth_header.is_none() {
                println!("❌ Header Authorization não encontrado");
                return Err(AuthError(AppMessage::new("Token de autenticação não informado.", 401)).into());
            }

            let auth_str = match auth_header.unwrap().to_str() {
                Ok(s) => s,
                Err(_) => {
                    println!("❌ Erro ao converter header para string");
                    return Err(AuthError(AppMessage::new("Header de autorização inválido.", 401)).into());
                }
            };

            println!("🔍 Header Authorization: {}", auth_str);

            // Divide o header "Bearer token"
            let parts: Vec<&str> = auth_str.split(' ').collect();
            if parts.len() != 2 || parts[0] != "Bearer" {
                println!("❌ Formato do token inválido. Partes: {:?}", parts);
                return Err(AuthError(AppMessage::new("Formato do token inválido.", 401)).into());
            }

            let token = parts[1];
            println!("🔑 Token extraído: {}...{}", &token[..20.min(token.len())],
                     if token.len() > 20 { &token[token.len()-10..] } else { "" });

            // Decodifica o token JWT
            let auth_config = AuthConfig::new();
            println!("🔐 Secret usado: {}...", &auth_config.secret[..10.min(auth_config.secret.len())]);

            let token_data = match decode::<Claims>(
                token,
                &DecodingKey::from_secret(auth_config.secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(data) => {
                    println!("✅ Token decodificado com sucesso");
                    data
                },
                Err(e) => {
                    println!("❌ Erro ao decodificar token: {:?}", e);

                    // Tentativa de debug adicional
                    match e.kind() {
                        jsonwebtoken::errors::ErrorKind::InvalidToken => {
                            println!("🔍 Token malformado");
                        },
                        jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                            println!("🔍 Assinatura inválida - possivelmente secret diferente");
                        },
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                            println!("🔍 Token expirado");
                        },
                        _ => {
                            println!("🔍 Outro erro de validação");
                        }
                    }

                    return Err(AuthError(AppMessage::new("Token JWT inválido.", 401)).into());
                }
            };

            // Parse do cliente do token
            let cliente_json = &token_data.claims.cliente;
            println!("👤 Cliente JSON: {}", cliente_json);

            let cliente_data: serde_json::Value = match serde_json::from_str(cliente_json) {
                Ok(data) => data,
                Err(e) => {
                    println!("❌ Erro ao parsear cliente JSON: {:?}", e);
                    return Err(AuthError(AppMessage::new("Dados do cliente inválidos no token.", 401)).into());
                }
            };

            let cliente_id = match cliente_data.get("id").and_then(|v| v.as_str()) {
                Some(id) => {
                    println!("✅ ID do cliente extraído: {}", id);
                    id
                },
                None => {
                    println!("❌ ID do cliente não encontrado no JSON: {}", cliente_data);
                    return Err(AuthError(AppMessage::new("ID do cliente não encontrado no token.", 401)).into());
                }
            };

            // Adiciona o cliente às extensions do request
            req.extensions_mut().insert(ClienteAuth { id: cliente_id.parse().unwrap() });

            println!("🚀 Autenticação bem-sucedida para cliente: {}", cliente_id);

            // Continua com o próximo middleware/handler
            service.call(req).await
        })
    }
}

// Função helper para extrair o cliente do request
pub fn get_cliente_from_request(req: &actix_web::HttpRequest) -> Result<ClienteAuth, AppMessage> {
    req.extensions()
        .get::<ClienteAuth>()
        .cloned()
        .ok_or_else(|| AppMessage::new("Cliente não autenticado", 401))
}
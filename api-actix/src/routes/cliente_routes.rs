use actix_web::web;
use crate::controllers::cliente_controller;

pub fn cliente_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(cliente_controller::create)
        .service(cliente_controller::login);
}
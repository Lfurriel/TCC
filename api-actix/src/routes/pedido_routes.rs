use actix_web::web;
use crate::controllers::pedido_controller;
use crate::middlewares::is_authenticated::Authentication;

pub fn pedido_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication)
            .service(pedido_controller::create)
    );
}
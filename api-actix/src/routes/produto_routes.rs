use actix_web::web;
use crate::controllers::produto_controller;

pub fn produto_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(produto_controller::get_all)
        .service(produto_controller::get_ofertas)
        .service(produto_controller::get_destaques)
        .service(produto_controller::get_by_categoria)
        .service(produto_controller::get_by_nome)
        .service(produto_controller::get_by_sku);
}
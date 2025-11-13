use actix_web::web;
use crate::controllers::categoria_controller;

pub fn categoria_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categoria_controller::get_all);
}
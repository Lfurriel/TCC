use actix_web::web;
use crate::controllers::home_controller;

pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home_controller::get_home_amazon).service(home_controller::get_home_shopee);
}
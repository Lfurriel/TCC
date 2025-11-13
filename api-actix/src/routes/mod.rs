use actix_web::{web};

pub mod categoria_routes;
pub mod produto_routes;
pub mod home_routes;
mod cliente_routes;
mod pedido_routes;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categorias")
            .configure(categoria_routes::categoria_routes)
    ).service(
        web::scope("/produtos")
            .configure(produto_routes::produto_routes)
    ).service(
        web::scope("/clientes")
            .configure(cliente_routes::cliente_routes)
    ).service(
        web::scope("/home")
            .configure(home_routes::home_routes)
    ).service(
        web::scope("/pedidos")
            .configure(pedido_routes::pedido_routes)
    );
}
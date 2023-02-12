mod api;
mod model;
mod repository;

use api::uri::{
    create_uri
};

use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    HttpServer::new( move || {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .service(create_uri)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}

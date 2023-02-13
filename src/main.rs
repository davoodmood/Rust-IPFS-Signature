mod api;
mod model;
mod repository;

use api::uri::create_uri;
use repository::attributes::populate_attributes;
use model::metadata::Attribute;
use crate::repository::hashtable::HashTable;

use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut dynamic_attributes: HashTable<Attribute> = HashTable::new(365, "attributes.bin");

    let attribute_check_object = Attribute {
        trait_type: Some("Flying Fish Tea Discount".to_string()),
        value: "20%".to_string(),
    };

    if !dynamic_attributes.search(&attribute_check_object) {
        let mut dynamic_attribute_array_placeholder = vec![Attribute {
            trait_type: Some("".to_string()),
            value: "".to_string(),
        }; 365];

        let dynamic_attribute_array = populate_attributes(&mut dynamic_attribute_array_placeholder);
        dynamic_attributes.fill(dynamic_attribute_array);
    }
    
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

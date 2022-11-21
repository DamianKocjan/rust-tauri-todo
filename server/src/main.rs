mod config;
mod database;
mod handlers;
mod models;
mod schema;
mod utils;

#[macro_use]
extern crate diesel;

use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{web, App, HttpServer};
use database::Database;

use config::config::load_config;
use handlers::todos::todos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let mut app = App::new()
            .wrap(cors)
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default());

        app = app.app_data(web::Data::new(Database::new(config.database.url.clone())));

        let mut api_scope = web::scope("/api/v1");
        api_scope = api_scope.service(todos::endpoints(web::scope("/todos")));

        app = app.service(api_scope);
        app
    })
    .bind("localhost:8000")?
    .run()
    .await
}

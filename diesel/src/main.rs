#[macro_use]
extern crate diesel;

mod middleware;
mod route;
mod lib;

use actix_web::{App, HttpServer}; 
use actix_web::{web::Data};

use std::sync::{Mutex};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = "localhost";
    let port = 11111; 
    let address = format!("{}:{}", host, port);

    let _ = listenfd::ListenFd::from_env();

    let db = Data::new(Mutex::new(lib::establish_connection()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::Logger::new())
            .service(route::index::helloworld)
            .service(route::index::foobar)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}

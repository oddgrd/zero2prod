use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Result};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a thread-safe reference counting pointer (Arc)
    let db_pool = web::Data::new(db_pool);

    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check::health_check)
            .service(subscriptions::subscribe)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

//! lib.rs

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .wait call, so it is not needed anymore.
pub fn run() -> Result<Server, std::io::Error> {
    // NOTE: no .await
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}

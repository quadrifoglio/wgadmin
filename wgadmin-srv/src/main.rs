//! wgadmin-srv

use actix_service::Service;
use actix_web::http::header::{HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN};
use actix_web::{web, App, HttpResponse, HttpServer};

use serde_json::json;

/// Handler for 'GET /api'.
/// Prints out version information.
async fn info() -> HttpResponse {
    let name = env!("CARGO_PKG_NAME");

    let version = format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    HttpResponse::Ok().json(json!({
        "name": name,
        "version": version,
    }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    res.headers_mut()
                        .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));

                    Ok(res)
                }
            })
            .route("/api", web::get().to(info))
            .route("/api/devices", web::get().to(device::list))
            .route("/api/devices/{name}", web::get().to(device::get))
            .route("/api/devices/{name}", web::post().to(device::post))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

mod device;
mod error;
mod peer;

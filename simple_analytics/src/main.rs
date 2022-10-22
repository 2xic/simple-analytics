use actix_web::{get,web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::{StatusCode};

async fn global(req: HttpRequest) -> Result<HttpResponse> {
    let userAgent = req.headers().get("User-Agent").unwrap().to_str()
    .unwrap();
    println!("user-agent : {}", userAgent);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("ok"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Running on http://localhost:{}/", port);

    HttpServer::new(|| {
        App::new().default_service(
        web::route().to(global)
    )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

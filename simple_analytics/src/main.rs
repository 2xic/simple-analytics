mod logger;
mod schema;
mod models;

extern crate diesel;
use diesel::sqlite::SqliteConnection;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result, web::{Data}};
use actix_web::http::{StatusCode};
use bytes::Bytes;
use diesel::r2d2::{ConnectionManager, Pool};


struct AppData {
    pool: Pool<ConnectionManager<SqliteConnection>>
}


async fn global(data: web::Data<AppData>, req: HttpRequest, body: Bytes) -> Result<HttpResponse> {
    let user_agent = req.headers().get("User-Agent").unwrap().to_str()
    .unwrap();
    let pool = &mut data.pool.get().unwrap();
    let metadata = body.escape_ascii().to_string();
    
    if let Some(val) = req.peer_addr() {
        println!("Request from ip: {:?}", val.ip());
        let ip = &Some(val.ip().to_string()).unwrap();
        logger::create_analytic(pool, user_agent, Some(ip), &metadata);
    } else{
        println!("Request from user-agent: {}", user_agent);
        logger::create_analytic(pool, user_agent, None, &metadata);
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "prod")]
    let port = 80;

    #[cfg(not(feature = "prod"))]
    let port = 8080;

    let pool = logger::establish_connection();

    println!("Running on http://localhost:{}/", port);

    HttpServer::new(move || {
        App::new().app_data(Data::new(AppData {
            pool: pool.clone()
        })).default_service(
        web::route().to(global)
    )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

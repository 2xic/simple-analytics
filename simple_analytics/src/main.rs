mod logger;
mod models;
mod schema;

extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result, web::{Data}};
use actix_web::http::{StatusCode};
use bytes::Bytes;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn create_analytic(conn: &mut SqliteConnection, value_user_agent: &str, value_ip: Option<&str>, value_metadata: &str) -> usize {
    use crate::schema::analytics;

    let entry = models::NewAnalytic { user_agent: value_user_agent, ip: value_ip.unwrap(), metadata: value_metadata };

    diesel::insert_into(analytics::table)
        .values(&entry)
        .execute(conn)
        .expect("Error saving new post")
}

struct AppData {
    pool: Pool<ConnectionManager<SqliteConnection>>
}


async fn global(data: web::Data<AppData>, req: HttpRequest, body: Bytes) -> Result<HttpResponse> {
    let user_agent = req.headers().get("User-Agent").unwrap().to_str()
    .unwrap();
    let app_name = &mut data.pool.get().unwrap();

    println!("user-agent : {}", user_agent);

    let metadata = body.escape_ascii().to_string();
    
    if let Some(val) = req.peer_addr() {
        println!("ip {:?}", val.ip());
        let ip = &Some(val.ip().to_string()).unwrap();
        create_analytic(app_name, user_agent, Some(ip), &metadata);
    } else{
        create_analytic(app_name, user_agent, None, &metadata);
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let connection = logger::establish_connection();

    println!("Running on http://localhost:{}/", port);

    HttpServer::new(move || {
        App::new().app_data(Data::new(AppData {
            pool: connection.clone()
        })).default_service(
        web::route().to(global)
    )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

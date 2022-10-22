mod logger;
mod models;
mod schema;

extern crate diesel;
use diesel::sqlite::SqliteConnection;
//use schema::analytics::dsl::*;
//use models::*;
use diesel::prelude::*;
//use models;
//use diesel_demo::*;
use actix_web::dev::Service;
use actix_web::{get,web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, web::{Data}};
use actix_web::http::{StatusCode};
use std::{cell::RefCell, sync::Mutex};
use models::{Analytic};

use diesel::r2d2::{ConnectionManager, Pool};

pub fn create_analytic(conn: &mut SqliteConnection, user_agent: &str) -> usize {
    use crate::schema::analytics;
    use schema::analytics::dsl::*;

    let new_analytic = models::NewAnalytic { user_agent: "test" };

    diesel::insert_into(analytics::table)
        .values(&new_analytic)
//        .load(conn)
        .execute(conn)
        .expect("Error saving new post")
}


struct app {
    pool: Pool<ConnectionManager<SqliteConnection>>
}


async fn global(data: web::Data<app>, req: HttpRequest) -> Result<HttpResponse> {
    let userAgent = req.headers().get("User-Agent").unwrap().to_str()
    .unwrap();
    /*
    let app_name = &data.app_name; // <- get app_name
    println!("app-name : {}", app_name);*/
    
    let app_name = &data.pool;
    print!("{:?}", app_name);

    println!("user-agent : {}", userAgent);
    
    if let Some(val) = req.peer_addr() {
        println!("ip {:?}", val.ip());
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("ok"))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let connection = /*mut*/ logger::establish_connection();

    println!("Running on http://localhost:{}/", port);

    
    //let m = Mutex::new(connection);
    
    HttpServer::new(move || {
        App::new().data(app {
            pool: connection.clone()
        }).default_service(
        web::route().to(global)
    )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

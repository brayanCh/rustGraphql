#![ allow(non_snake_case)]

mod resolvers;
mod db;

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use resolvers::{ sendString };
use db::{ initMongoConnection };

#[get("/graphql")]
async fn greet() -> web::Json<String>
{ 
    print!("{}", sendString());
    return web::Json(sendString());
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let a = initMongoConnection();
    match a.await {
        Ok(()) => {
            println!("It worked");
        },
        Err(err) => {
            println!("{}", err);
        }
    }

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


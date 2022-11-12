#![ allow(non_snake_case)]

mod resolvers;

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use resolvers::{ sendString };


#[get("/graphql")]
async fn greet() -> web::Json<String>
{ 
    print!("{}", sendString());
    return web::Json(sendString());
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


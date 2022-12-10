#![ allow(non_snake_case)]

mod resolvers;
mod db;
mod schemas;
mod endpoint;

use actix_web::{post, web, App, get, HttpServer, HttpResponse };
use juniper::http::{ graphiql::graphiql_source };
use db::{ initMongoConnection };
use schemas::user::{ UserSchema };
use endpoint::{ mainEndpoint };
//use mongodb::{ Database };


 
#[get("/graphiql")]
async fn graphQlInterface() -> HttpResponse
{

    let html = graphiql_source(&"/graphql", Some("ws://localhost:8080"));

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let a = initMongoConnection();
    match a.await {
        Ok(db) => {
            let listCollections = db.list_collection_names(None).await.expect("ddd");
            for i in listCollections
            {
                println!("{}", i);
            }
            println!("It worked");
        },
        Err(err) => {
            println!("{}", err);
        }
    }
    HttpServer::new(|| {
        App::new()
            .service(mainEndpoint)
            .service(graphQlInterface)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


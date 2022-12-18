#![ allow(non_snake_case)]

mod resolvers;
mod db;
mod schemas;
mod endpoint;

use actix_web::{post, web, App, get,  HttpServer, HttpResponse };
use juniper::http::{ graphiql::graphiql_source };
use db::{ initMongoConnection };
use schemas::user::{ UserSchema };
use resolvers::userResolver::{ newSchema, Schema };
use endpoint::{ mainEndpoint };
use mongodb::{ Database };


 
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
    let schema = newSchema();
    let schemaPassed = web::Data::new(schema);

    let mongoClient = initMongoConnection().await.unwrap();
    let dataMongo = web::Data::new(mongoClient);

   
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&schemaPassed))
            .app_data(web::Data::clone(&dataMongo))
            .service(mainEndpoint)
            .service(graphQlInterface)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#![ allow(non_snake_case)]

mod resolvers;
mod db;

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use resolvers::{ sendString };
use db::{ initMongoConnection };
use serde::Serialize;

#[derive(Serialize)]
struct User {
    ID: String,
    name: String,
    email : String,
    cellnumber : String,
    profilePicUrl : String,
    planType : String,
    registerDay: i32,
    lastPaymentDay: i32,
    hasCancelledTheService: bool
}




#[get("/graphql")]
async fn greet() -> web::Json<Vec<User>>
{
    let mut returnedJSON :Vec<User>  = Vec::new(); 

    returnedJSON.push(User{
        ID: "215414asfdasdfg2354".to_string(),
        name: "John Smith".to_string(),
        email: "JohnSmith@gmail.com".to_string(),
        cellnumber: "+233201244474".to_string(),
        profilePicUrl: "dfadgfag3qwtqsgfda313".to_string(),
        planType: "Standard".to_string(),
        registerDay: 1234146i32,
        lastPaymentDay: 1234146i32,
        hasCancelledTheService: false
    });

    return web::Json(returnedJSON);
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


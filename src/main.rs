#![ allow(non_snake_case)]

mod resolvers;
mod db;
mod schemas;

use actix_web::{post, web, App, HttpServer };
use db::{ initMongoConnection };
use schemas::user::{ User };


#[post("/graphql")]
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
            .route("/hello", web::post().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


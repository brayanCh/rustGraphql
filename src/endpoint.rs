use actix_web::{ web, post, HttpResponse };
use crate::schemas::user::{ UserSchema };
use juniper::http::{ GraphQLRequest };
use crate::resolvers::userResolver::{ Schema, Context };

#[post("/graphql")]
pub async fn mainEndpoint(
        data: web::Json<GraphQLRequest>,
        schema: web::Data<Schema>
    ) -> HttpResponse
{
    let mut returnedJSON :Vec<UserSchema>  = Vec::new(); 

    returnedJSON.push(UserSchema{
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
    let newContext : Context = Context { stData : "XD".to_string() };

    let retVal = data.execute(&schema, &newContext).await;

    return HttpResponse::Ok().json(retVal);
}

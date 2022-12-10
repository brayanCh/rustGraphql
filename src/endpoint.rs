use actix_web::{ web, post };
use crate::{ schemas };


#[post("/graphql")]
pub async fn mainEndpoint() -> web::Json<Vec<schemas::user::UserSchema>>
{
    let mut returnedJSON :Vec<schemas::user::UserSchema>  = Vec::new(); 

    returnedJSON.push(schemas::user::UserSchema{
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

use serde::{ Serialize, Deserialize };
use juniper::{ GraphQLObject, GraphQLInputObject };

//let db = client.database("mydb")

#[derive(Serialize, Debug, GraphQLObject, Deserialize)]
pub struct UserSchema {
    pub ID: String,
    pub name: String,
    pub email : String,
    pub cellnumber : String,
    pub profilePicUrl : String,
    pub planType : String,
    pub registerDay: i32,
    pub lastPaymentDay: i32,
    pub hasCancelledTheService: bool
}

#[derive( GraphQLInputObject)]
pub struct CreateUserInput {
    pub ID: String,
    pub name: String,
    pub email : String,
    pub cellnumber : String,
    pub profilePicUrl : String,
    pub planType : String,
    pub registerDay: i32,
    pub lastPaymentDay: i32,
}


use serde::Serialize;
use mongodb::{ Database, Collection };
use juniper::GraphQLObject;

//let db = client.database("mydb")

#[derive(Serialize, GraphQLObject)]
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

pub fn userCollection (db : &Database ) -> Collection<UserSchema>
{
    return db.collection::<UserSchema>("user");
}

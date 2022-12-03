use serde::Serialize;
use mongodb::{ Database, Collection };

//let db = client.database("mydb")

#[derive(Serialize)]
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

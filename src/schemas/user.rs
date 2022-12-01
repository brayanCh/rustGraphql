use serde::Serialize;


#[derive(Serialize)]
pub struct User {
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

use serde::Serialize;
use juniper::{ GraphQLObject };

#[derive(Serialize, GraphQLObject)]
pub struct EmployeeSchema {
    pub ID: String,
    pub name: String,
    pub email : String,
    pub cellnumber : String,
    pub profilePicUrl : String,
    pub monthlyWage: i32,
    pub lastWageDay: i32,
    pub dayBeginningVacations : i32,
    pub lengthVacations : i32,
    pub registerDay: i32,
    pub isStillEmployed: bool,
    pub isInVacation: bool,
    pub isAdmin: bool
}



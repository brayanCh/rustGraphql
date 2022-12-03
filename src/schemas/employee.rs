use serde::Serialize;
use mongodb::{ Database, Collection };

#[derive(Serialize)]
pub struct EmployeeSchema {
    pub ID: String,
    pub name: String,
    pub email : String,
    pub cellnumber : String,
    pub profilePicUrl : String,
    pub monthlyWage: i16,
    pub lastWageDay: i32,
    pub dayBeginningVacations : i32,
    pub lengthVacations : i32,
    pub registerDay: i32,
    pub isStillEmployed: bool,
    pub isInVacation: bool,
    pub isAdmin: bool
}


pub fn employeeCollection (db : &Database ) -> Collection<EmployeeSchema>
{
    return db.collection::<EmployeeSchema>("employee");
}


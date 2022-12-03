use serde::Serialize;
use mongodb::{ Database, Collection };


#[derive(Serialize)]
pub struct EquipmentSchema {
    pub ID: String,
    pub name: String,
    pub price : i16,
    pub purchaseDay: i32,
    pub expectedMaintenanceDate: i32,
    pub hasBeenThrowAway: bool,
    pub isInMaintenance: bool
}


pub fn equipmentCollection (db : &Database ) -> Collection<EquipmentSchema>
{
    return db.collection::<EquipmentSchema>("equipment");
}

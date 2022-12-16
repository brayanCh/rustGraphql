//mod schemas;
use mongodb::{ Database, Collection };
use juniper::{ RootNode, EmptyMutation, FieldError , EmptySubscription};
use crate::schemas::user::{ UserSchema };

pub struct Context {
    pub stData: String
}

impl juniper::Context for Context {}

pub struct Query {}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn newSchema() -> Schema {
    Schema::new(Query{}, EmptyMutation::<Context>::new(), EmptySubscription::<Context>::new() )
}



#[juniper::graphql_object(
    Context = Context 
)]
impl Query
{
    pub fn user() -> Result<Vec<UserSchema>, FieldError>
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

        return Ok(returnedJSON);
    }
    pub fn getString () -> &str 
    {
        return "dddd";
    }

}

/*
pub struct userResolver {}

impl userResolver
{
    pub async fn getAllUsers()
    {
        
    }
}
*/

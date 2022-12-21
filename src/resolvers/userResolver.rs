//mod schemas;
use mongodb::{ Database, Collection };
use juniper::{ RootNode, EmptyMutation, FieldError , EmptySubscription};
use crate::schemas::user::{ UserSchema, CreateUserInput };
use std::sync::{ Arc };
use mongodb::bson::{doc, Document};

pub struct Context {
    pub database: Arc<Database>
}

impl juniper::Context for Context {}

pub struct Query {}

pub struct Mutation {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn newSchema() -> Schema {
    Schema::new(Query{}, Mutation{}, EmptySubscription::<Context>::new() )
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

    pub async fn getAllUsers(context : &Context) -> Result<Vec<UserSchema>, FieldError>
    {
        let collUser = context.database.collection::<UserSchema>("user");
        let mut res = collUser.find(None, None).await?;

        let mut returnedJSON :Vec<UserSchema>  = Vec::new(); 

        while res.advance().await?
        {
            returnedJSON.push(res.deserialize_current()?);
        }

        return Ok(returnedJSON);
    }
}

#[juniper::graphql_object(
    Context = Context 
)]
impl Mutation 
{
    pub async fn createUser(input : CreateUserInput, context : &Context) -> Result<UserSchema, FieldError>
    {

        let collUser = context.database.collection::<UserSchema>("user");

        let res = UserSchema{
            ID: input.ID.to_string(),
            name: input.name.to_string(),
            email: input.email.to_string(),
            cellnumber: input.cellnumber.to_string(),
            profilePicUrl: input.profilePicUrl.to_string(),
            planType: input.planType.to_string(),
            registerDay: input.registerDay,
            lastPaymentDay: input.lastPaymentDay,
            hasCancelledTheService: false
        };

        let response = collUser.insert_one(&res, None).await;
        println!("{:?}", &response);

        return Ok(res);
    }
}

use bson::oid::ObjectId;
use juniper::{graphql_object, GraphQLInputObject};
use serde::{Deserialize, Serialize};

use crate::schemas::root::Context;

/// User structures that we use to handle Juniper with MongoDB.
/// We are forced to create a `MongoUser` and a `JuniperUser` because the Juniper crate is still leveraging
/// the bson crate at version 1.x. By creating this bridge we ensure compatibility and an easier transition when
/// Juniper updates (coming with the 0.16 that is 90% complete on Github).
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MongoUser {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct JuniperUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<MongoUser> for JuniperUser {
    fn from(mongo_user: MongoUser) -> Self {
        Self {
            id: mongo_user.id.to_hex(),
            name: mongo_user.name,
            email: mongo_user.email,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[graphql_object(Context = Context)]
impl JuniperUser {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }
}

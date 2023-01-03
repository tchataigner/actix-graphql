use crate::schemas::user::{JuniperUser, MongoUser, UserInput};
use bson::oid::ObjectId;
use bson::{doc, Bson};
use futures::TryStreamExt;
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};

pub struct Context {
    pub mongo_client: mongodb::Client,
    pub mongo_database: String,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    async fn users(context: &Context) -> FieldResult<Vec<JuniperUser>> {
        let conn = context
            .mongo_client
            .database(&context.mongo_database)
            .collection::<MongoUser>("users");

        let users_documents = match conn.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Ok(vec![]),
        };
        let users: Vec<JuniperUser> = users_documents
            .map_ok(|document| {
                dbg!(&document);
                return JuniperUser::from(document);
            })
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![]);

        dbg!(&users);
        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    async fn user(context: &Context, id: String) -> FieldResult<JuniperUser> {
        let conn = context
            .mongo_client
            .database(&context.mongo_database)
            .collection::<MongoUser>("users");

        let bson_id = Bson::ObjectId(ObjectId::parse_str(&id).unwrap());

        let user = conn
            .find_one(doc! { "_id": bson_id }, None)
            .await
            .unwrap()
            .unwrap();

        Ok(JuniperUser::from(user))
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, user: UserInput) -> FieldResult<JuniperUser> {
        let conn = context
            .mongo_client
            .database(&context.mongo_database)
            .collection("users");

        let insert_result = conn
            .insert_one(doc! { "name": &user.name, "email": &user.email }, None)
            .await
            .unwrap();

        let inserted_id = match insert_result.inserted_id {
            Bson::ObjectId(object_id) => object_id,
            _ => panic!("inserted_id should be an Object Id"),
        };

        Ok(JuniperUser {
            id: inserted_id.to_hex(),
            name: user.name,
            email: user.email,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}

use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use crate::models::{Context, PublicUser, User};
// use juniper::{EmptyMutation, GraphQLEnum, GraphQLInputObject, GraphQLObject};

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {

	async fn get_user_by_id(
		context: &Context,
		#[graphql(description = "id of the user")] id: i32,
	) -> FieldResult<PublicUser> {
		let user = User::get_by_id(id, &context.db).await;
		Ok(PublicUser::from_user(user.unwrap()))
	}

	async fn get_all_users(
		context: &Context
	) -> FieldResult<Vec<PublicUser>> {
		let users = User::get_all(&context.db).await;
		Ok(
			users
				.unwrap()
				.into_iter()
				.map(|u| PublicUser::from_user(u))
				.collect()
		)
	}

}

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {

    async fn get_id(
		_context: &Context,
		#[graphql(description = "number to get")] id: i32,
	) -> FieldResult<i32> {
        Ok(id)
    }
	
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::<Context>::new())
}

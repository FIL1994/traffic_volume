use juniper::FieldResult;
use juniper::RootNode;

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

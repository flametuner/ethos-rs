use async_graphql::{Context, EmptySubscription, Object, Schema};

use crate::{
    errors::StoreError,
    services::project::{Project, ProjectService},
};

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello world".to_string()
    }

    async fn projects<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Project>, StoreError> {
        let service = ctx.data::<ProjectService>().unwrap();
        service.get_projects()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_project<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
        description: Option<String>,
    ) -> Result<Project, StoreError> {
        let service = ctx.data::<ProjectService>().unwrap();
        service.create_project(name, description)
    }
}

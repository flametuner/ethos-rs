use async_graphql::{async_trait, Context, Error, Guard};

use crate::services::project::Project;

pub struct WithProject;

#[async_trait::async_trait]
impl Guard for WithProject {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        if let Some(_wallet) = ctx.data_opt::<Project>() {
            return Ok(());
        }
        Err("You need a valid `project` value in your headers".into())
    }
}

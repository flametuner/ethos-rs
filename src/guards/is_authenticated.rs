use async_graphql::{async_trait, Context, Error, Guard};

use crate::services::wallet::Wallet;

pub struct IsAuthenticated;

#[async_trait::async_trait]
impl Guard for IsAuthenticated {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        if let Some(_wallet) = ctx.data_opt::<Wallet>() {
            return Ok(());
        }
        Err("Forbidden".into())
    }
}

use async_graphql::{Context, EmptySubscription, Object, Schema};
use ethabi::Address;
use std::str::FromStr;

use crate::{
    errors::StoreError,
    services::{
        project::{Project, ProjectService},
        wallet::{Wallet, WalletService},
    },
};

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> String {
        "ok".to_string()
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

    async fn wallet<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        address: String,
    ) -> Result<Wallet, StoreError> {
        let address = Address::from_str(&address).map_err(|_e| StoreError::InvalidAddress)?;
        let service = ctx.data::<WalletService>().unwrap();
        service.upsert_wallet(address)
    }
}

use crate::guards::with_project::WithProject;
use crate::{
    guards::is_authenticated::IsAuthenticated,
    services::{
        nft::{Collection, NftService},
        profile::UpdateProfileInput,
    },
};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use ethers::types::Address;
use std::{str::FromStr, sync::Arc};

use crate::{
    errors::StoreError,
    services::{
        auth::{AuthService, LoginResponse},
        profile::{Profile, ProfileService},
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

    #[graphql(guard = "IsAuthenticated")]
    async fn profile<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Profile, StoreError> {
        let wallet = ctx.data_unchecked::<Wallet>();
        let service = ctx.data::<ProfileService>().unwrap();
        service.get_profile(wallet)
    }

    #[graphql(guard = "WithProject")]
    async fn collections<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Collection>, StoreError> {
        let service = ctx.data::<NftService>().unwrap();
        let project = ctx.data::<Project>().unwrap();
        service.get_collections(project)
    }

    async fn projects<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Project>, StoreError> {
        let service = ctx.data::<Arc<ProjectService>>().unwrap();
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
        let service = ctx.data::<Arc<ProjectService>>().unwrap();
        service.create_project(&name, description)
    }

    #[graphql(guard = "IsAuthenticated")]
    async fn update_profile<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: UpdateProfileInput,
    ) -> Result<Profile, StoreError> {
        let wallet = ctx.data_unchecked::<Wallet>();
        let service = ctx.data::<ProfileService>().unwrap();
        let profile = service.update_profile(wallet, input.name, input.email)?;
        Ok(profile)
    }

    async fn wallet<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        address: String,
    ) -> Result<Wallet, StoreError> {
        let address = Address::from_str(&address)?;
        let service = ctx.data::<Arc<WalletService>>().unwrap();
        service.upsert_wallet(address)
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        address: String,
        signature: String,
    ) -> Result<LoginResponse, StoreError> {
        let address = Address::from_str(&address)?;
        let service = ctx.data::<Arc<AuthService>>().unwrap();
        service.login(address, signature).await
    }
}

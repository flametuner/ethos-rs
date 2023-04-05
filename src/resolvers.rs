use crate::guards::with_project::WithProject;
use crate::services::nft::{FilterNFTsInput, PaginatedNFTs};
use crate::{
    guards::is_authenticated::IsAuthenticated,
    services::{
        nft::{Collection, NftService},
        profile::UpdateProfileInput,
    },
};
use async_graphql::{Context, Object};
use ethers::types::Address;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
    errors::EthosError,
    services::{
        auth::{AuthService, LoginResponse},
        profile::{Profile, ProfileService},
        project::{Project, ProjectService},
        wallet::{Wallet, WalletService},
    },
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> String {
        "ok".to_string()
    }

    #[graphql(guard = "IsAuthenticated")]
    async fn profile<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Profile, EthosError> {
        let wallet = ctx.data_unchecked::<Wallet>();
        let service = ctx.data::<ProfileService>().unwrap();
        service.get_profile(wallet)
    }

    #[graphql(guard = "WithProject")]
    async fn collections<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Collection>, EthosError> {
        let service = ctx.data::<NftService>().unwrap();
        let project = ctx.data::<Project>().unwrap();
        service.get_collections(project)
    }

    async fn projects<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Project>, EthosError> {
        let service = ctx.data::<Arc<ProjectService>>().unwrap();
        service.get_projects()
    }

    async fn collection<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Uuid,
    ) -> Result<Collection, EthosError> {
        let service = ctx.data::<NftService>().unwrap();
        service.get_collection(id)
    }

    async fn nfts<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FilterNFTsInput,
    ) -> Result<PaginatedNFTs, EthosError> {
        let service = ctx.data::<NftService>().unwrap();
        service.get_nfts(input)
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
    ) -> Result<Project, EthosError> {
        let service = ctx.data::<Arc<ProjectService>>().unwrap();
        service.create_project(&name, description)
    }

    #[graphql(guard = "IsAuthenticated")]
    async fn update_profile<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: UpdateProfileInput,
    ) -> Result<Profile, EthosError> {
        let wallet = ctx.data_unchecked::<Wallet>();
        let service = ctx.data::<ProfileService>().unwrap();
        let profile = service.update_profile(wallet, input.name, input.email)?;
        Ok(profile)
    }

    async fn wallet<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        address: String,
    ) -> Result<Wallet, EthosError> {
        let address = Address::from_str(&address)?;
        let service = ctx.data::<Arc<WalletService>>().unwrap();
        service.upsert_wallet(address)
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        address: String,
        signature: String,
    ) -> Result<LoginResponse, EthosError> {
        let address = Address::from_str(&address)?;
        let service = ctx.data::<Arc<AuthService>>().unwrap();
        service.login(address, signature).await
    }
}

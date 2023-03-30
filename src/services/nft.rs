use async_graphql::{Enum, InputObject, SimpleObject};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{Identifiable, PgConnection, Queryable};
use r2d2::Pool;
use uuid::Uuid;

use crate::database::ConnectionPool;
use crate::errors::StoreError;
use crate::schema::{
    attributes_on_nfts, collection_contracts, collections, networks, nft_attributes, nfts,
};

use super::project::Project;
use super::wallet::Wallet;

#[derive(Debug, Queryable, SimpleObject, Identifiable)]
#[diesel(table_name = networks)]
pub struct Network {
    id: Uuid,
    chain_id: i32,
}

#[derive(Debug, Queryable, SimpleObject, Identifiable, Associations)]
#[diesel(table_name = collection_contracts)]
#[diesel(belongs_to(Collection))]
#[diesel(belongs_to(Network))]
pub struct CollectionContract {
    pub id: Uuid,
    // contract id on bifrost
    contract_id: Option<Uuid>,
    // fee recipient address
    fee_recipient: String,

    address: String,

    // relations
    collection_id: Uuid,
    network_id: Uuid,
}

#[derive(Debug, Queryable, SimpleObject, Associations, Identifiable)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = collections)]
pub struct Collection {
    pub id: Uuid,
    name: String,
    description: Option<String>,
    image: Option<String>,
    external_link: Option<String>,
    seller_fee_basis_points: Option<i32>,
    project_id: Uuid,

    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = nfts)]
pub struct NewNft {
    pub nft_id: i32,
    pub name: String,
    pub image: String,
    pub description: String,
    pub external_url: String,
    pub animation_url: String,

    pub collection_id: Uuid,
    pub network_contract_id: Uuid,
}

#[derive(Debug, Queryable, SimpleObject, Associations, Identifiable)]
#[diesel(belongs_to(Wallet, foreign_key = owner_id))]
#[diesel(belongs_to(CollectionContract, foreign_key = network_contract_id))]
#[diesel(belongs_to(Collection))]
#[diesel(table_name = nfts)]
pub struct Nft {
    id: Uuid,
    nft_id: i32,
    name: String,
    description: String,
    minted_at: Option<chrono::NaiveDateTime>,
    image: String,
    external_url: String,
    animation_url: String,

    owner_id: Option<Uuid>,
    collection_id: Uuid,
    network_contract_id: Uuid,
}

#[derive(Queryable, SimpleObject, Associations, Identifiable)]
#[diesel(belongs_to(NftAttributes, foreign_key = attribute_id))]
#[diesel(belongs_to(Nft))]
#[diesel(primary_key(nft_id, attribute_id))]
#[diesel(table_name = attributes_on_nfts)]
struct AttributesOnNft {
    nft_id: Uuid,
    attribute_id: Uuid,
}

#[derive(Queryable, SimpleObject, Identifiable)]
#[diesel(table_name = nft_attributes)]
struct NftAttributes {
    id: Uuid,
    trait_type: Option<String>,
    value: Option<String>,
    max_value: Option<u32>,
    display_type: Option<DisplayType>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum DisplayType {
    Number,
    BoostPercentage,
    BoostNumber,
}

#[derive(SimpleObject)]
pub struct PaginatedNFTs {
    edges: Vec<Nft>,
    total_count: i32,
    next_cursor: Option<Uuid>,
}

#[derive(InputObject)]
pub struct FilterNFTsInput {
    nft_id: Option<i32>,
    take: Option<i32>,
    cursor: Option<Uuid>,
    collection_id: Option<Uuid>,

    tier: Option<i32>,
    minted: Option<bool>,
    order_by: Option<NFTOrderBy>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Sort {
    Asc,
    Desc,
}

#[derive(InputObject)]
pub struct NFTOrderBy {
    nft_id: Sort,
    minted: Sort,
}

pub struct NftService {
    pool: ConnectionPool,
}

impl NftService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn create_collection(
        &self,
        project: &Project,
        name_str: &str,
        desc: Option<String>,
    ) -> Result<Collection, StoreError> {
        use crate::schema::collections::dsl::*;
        let mut conn = self.pool.get()?;

        let result = diesel::insert_into(collections)
            .values((
                name.eq(name_str),
                project_id.eq(project.id),
                description.eq(desc),
            ))
            .get_result::<Collection>(&mut conn)?;
        Ok(result)
    }

    pub fn get_collections(&self, project: &Project) -> Result<Vec<Collection>, StoreError> {
        let mut conn = self.pool.get()?;

        let result = Collection::belonging_to(&project).load::<Collection>(&mut conn)?;
        Ok(result)
    }

    pub fn get_collection(&self, id: Uuid) -> Result<Collection, StoreError> {
        use crate::schema::collections::dsl::collections;
        let mut conn = self.pool.get()?;

        let result = collections.find(id).first(&mut conn)?;
        Ok(result)
    }

    pub fn get_collection_by_name(
        &self,
        project: &Project,
        name_str: &str,
    ) -> Result<Collection, StoreError> {
        use crate::schema::collections::dsl::*;
        let mut conn = self.pool.get()?;

        let result = collections
            .filter(project_id.eq(project.id))
            .filter(name.eq(name_str))
            .first::<Collection>(&mut conn)?;
        Ok(result)
    }

    pub fn create_network(&self, chain: i32) -> Result<Network, StoreError> {
        use crate::schema::networks::dsl::*;
        let mut conn = self.pool.get()?;

        let result = diesel::insert_into(networks)
            .values(chain_id.eq(chain))
            .get_result::<Network>(&mut conn)?;
        Ok(result)
    }

    pub fn get_network_by_id(&self, chain: i32) -> Result<Network, StoreError> {
        use crate::schema::networks::dsl::*;
        let mut conn = self.pool.get()?;

        let result = networks
            .filter(chain_id.eq(chain))
            .first::<Network>(&mut conn)?;
        Ok(result)
    }

    pub fn create_collection_contract(
        &self,
        collection: &Collection,
        network: &Network,
        addr: &str,
        recipient: &str,
    ) -> Result<CollectionContract, StoreError> {
        use crate::schema::collection_contracts::dsl::*;
        let mut conn = self.pool.get()?;

        println!("{:?}", collection);
        println!("{:?}", network);
        let result = diesel::insert_into(collection_contracts)
            .values((
                network_id.eq(network.id),
                collection_id.eq(collection.id),
                address.eq(addr),
                fee_recipient.eq(recipient),
            ))
            .get_result::<CollectionContract>(&mut conn)?;
        println!("{:?}", result.collection_id);
        println!("{:?}", result.network_id);
        Ok(result)
    }

    pub fn get_collection_contract_by_address(
        &self,
        network: &Network,
        addr: &str,
    ) -> Result<CollectionContract, StoreError> {
        use crate::schema::collection_contracts::dsl::*;
        let mut conn = self.pool.get()?;

        let result = collection_contracts
            .filter(address.eq(addr))
            .filter(network_id.eq(network.id))
            .first::<CollectionContract>(&mut conn)?;
        Ok(result)
    }

    pub fn create_nfts(&self, nft_list: Vec<NewNft>) -> Result<Vec<Nft>, StoreError> {
        use crate::schema::nfts::dsl::*;
        let mut conn = self.pool.get()?;

        let result = diesel::insert_into(nfts)
            .values(nft_list)
            .get_results::<Nft>(&mut conn)?;
        Ok(result)
    }
}

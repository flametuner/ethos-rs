use async_graphql::{Enum, SimpleObject};
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

#[derive(Queryable, SimpleObject, Identifiable)]
#[diesel(table_name = networks)]
struct Network {
    id: Uuid,
    chain_id: i32,
}

#[derive(Queryable, SimpleObject, Identifiable, Associations)]
#[diesel(table_name = collection_contracts)]
#[diesel(belongs_to(Network))]
#[diesel(belongs_to(Collection))]
struct CollectionContract {
    id: Uuid,
    // contract id on bifrost
    contract_id: String,
    // fee recipient address
    fee_recipient: String,

    address: String,

    // relations
    network_id: Uuid,
    collection_id: Uuid,
}

#[derive(Queryable, SimpleObject, Associations, Identifiable)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = collections)]
pub struct Collection {
    id: Uuid,
    name: String,
    description: Option<String>,
    image: Option<String>,
    external_link: Option<String>,
    seller_fee_basis_points: Option<i32>,
    project_id: Uuid,

    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, SimpleObject, Associations, Identifiable)]
#[diesel(belongs_to(Wallet, foreign_key = owner_id))]
#[diesel(belongs_to(CollectionContract, foreign_key = network_contract_id))]
#[diesel(belongs_to(Collection))]
#[diesel(table_name = nfts)]
struct Nft {
    id: Uuid,
    nft_id: u32,
    name: String,
    image: String,
    description: String,
    minted: bool,
    minted_at: chrono::NaiveDateTime,
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

pub struct CollectionService {
    pool: ConnectionPool,
}

impl CollectionService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn get_collections(&self, project: &Project) -> Result<Vec<Collection>, StoreError> {
        let mut conn = self.pool.get()?;

        let result = Collection::belonging_to(&project).load::<Collection>(&mut conn)?;
        Ok(result)
    }
}

pub struct NetworkService {
    pool: ConnectionPool,
}

impl NetworkService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn get_network_by_id(&self, chain: i32) -> Result<Network, StoreError> {
        use crate::schema::networks::dsl::*;
        let mut conn = self.pool.get()?;

        let result = networks
            .filter(chain_id.eq(chain))
            .first::<Network>(&mut conn)?;
        Ok(result)
    }
}

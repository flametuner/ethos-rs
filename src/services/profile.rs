use crate::database::ConnectionPool;
use crate::guards::is_authenticated::IsAuthenticated;
use crate::schema::profiles;
use crate::services::wallet::Wallet;
use async_graphql::{ComplexObject, Context};
use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::Queryable;
use ethers::types::Address;
use r2d2::Pool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::EthosError;

use super::wallet::WalletService;

#[derive(Debug, SimpleObject, Queryable, Associations, Identifiable)]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = profiles)]
#[graphql(complex)]
pub struct Profile {
    pub id: Uuid,
    name: Option<String>,
    email: Option<String>,
    #[graphql(skip)]
    wallet_id: Uuid,
}

#[ComplexObject]
impl Profile {
    #[graphql(guard = "IsAuthenticated")]
    pub async fn wallet(&self, ctx: &Context<'_>) -> Result<Wallet, EthosError> {
        let wallet = ctx.data_unchecked::<Wallet>();

        let wallet_service = ctx.data::<Arc<WalletService>>().unwrap();
        let addr = Address::from_str(&wallet.address)?;
        let wallet = wallet_service.get_wallet(&addr)?;
        Ok(wallet)
    }
}

pub struct ProfileService {
    pool: ConnectionPool,
}

#[derive(Insertable, Default)]
#[diesel(table_name = profiles)]
struct NewProfile {
    wallet_id: Uuid,
    name: Option<String>,
    email: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateProfileInput {
    pub name: Option<String>,
    #[graphql(validator(email))]
    pub email: Option<String>,
}

impl ProfileService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn new_profile(&self, wallet: Uuid) -> Result<Profile, EthosError> {
        use crate::schema::profiles::dsl::*;
        let mut conn = self.pool.get()?;
        let new_profile = NewProfile {
            wallet_id: wallet,
            ..NewProfile::default()
        };
        Ok(diesel::insert_into(profiles)
            .values(&new_profile)
            .get_result::<Profile>(&mut conn)?)
    }

    pub fn get_profile(&self, wallet: &Wallet) -> Result<Profile, EthosError> {
        let mut conn = self.pool.get()?;

        let profile = Profile::belonging_to(&wallet).first(&mut conn).optional()?;

        match profile {
            Some(profile) => Ok(profile),
            None => Ok(self.new_profile(wallet.id)?),
        }
    }

    pub fn update_profile(
        &self,
        wallet: &Wallet,
        name_input: Option<String>,
        email_input: Option<String>,
    ) -> Result<Profile, EthosError> {
        use crate::schema::profiles::dsl::*;
        let mut conn = self.pool.get()?;
        let profile = diesel::update(profiles)
            .filter(wallet_id.eq(wallet.id))
            .set((name.eq(name_input), email.eq(email_input)))
            .get_result(&mut conn)?;
        Ok(profile)
    }
}

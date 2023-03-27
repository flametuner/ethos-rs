use crate::database::ConnectionPool;
use crate::schema::profiles;
use crate::services::wallet::Wallet;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::Queryable;
use r2d2::Pool;
use uuid::Uuid;

use crate::errors::StoreError;

#[derive(Debug, SimpleObject, Queryable, Associations, Identifiable)]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: Uuid,
    name: Option<String>,
    email: Option<String>,
    #[graphql(skip)]
    wallet_id: Uuid,
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

impl ProfileService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn new_profile(&self, wallet: Uuid) -> Result<Profile, StoreError> {
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

    pub fn get_profile(&self, wallet: &Wallet) -> Result<Profile, StoreError> {
        let mut conn = self.pool.get()?;

        let profile = Profile::belonging_to(&wallet).first(&mut conn).optional()?;

        match profile {
            Some(profile) => Ok(profile),
            None => Ok(self.new_profile(wallet.id)?),
        }
    }
}

use crate::database::ConnectionPool;
use crate::schema::profiles;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{QueryDsl, Queryable};
use r2d2::Pool;
use uuid::Uuid;

use crate::errors::StoreError;

#[derive(Debug, SimpleObject, Queryable)]
pub struct Profile {
    pub id: Uuid,
    name: Option<String>,
    email: Option<String>,
}

pub struct ProfileService {
    pool: ConnectionPool,
}

#[derive(Insertable, Default)]
#[diesel(table_name = profiles)]
struct NewProfile {
    name: Option<String>,
    email: Option<String>,
}

impl ProfileService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn new_profile(&self) -> Result<Profile, StoreError> {
        use crate::schema::profiles::dsl::*;
        let mut conn = self.pool.get()?;
        Ok(diesel::insert_into(profiles)
            .values(&NewProfile::default())
            .get_result::<Profile>(&mut conn)?)
    }

    pub fn get_profile(&self, profile_id: Uuid) -> Result<Profile, StoreError> {
        use crate::schema::profiles::dsl::*;
        let mut conn = self.pool.get()?;

        let profile = profiles.find(profile_id).first(&mut conn)?;
        Ok(profile)
    }
}

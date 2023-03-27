use crate::database::ConnectionPool;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{QueryDsl, Queryable};
use r2d2::Pool;
use uuid::Uuid;

use crate::errors::StoreError;

#[derive(Debug, SimpleObject, Queryable)]
pub struct Profile {
    id: Uuid,
    name: String,
    email: String,
}

pub struct ProfileService {
    pool: ConnectionPool,
}

impl ProfileService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn get_profile(&self, profile_id: Uuid) -> Result<Profile, StoreError> {
        use crate::schema::profiles::dsl::*;
        let mut conn = self.pool.get()?;

        let profile = profiles.find(profile_id).first(&mut conn)?;
        Ok(profile)
    }
}

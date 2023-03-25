use crate::{database::ConnectionPool, schema::profiles};
use async_graphql::SimpleObject;
use diesel::Queryable;
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
    fn get_profile(&self, profile_id: String) -> Result<Profile, StoreError> {
        use crate::schema::profiles::dsl::*;
        let conn = self.pool.get()?;
        // let profile = profiles.filter(id.eq(profile_id)).get_result(&mut conn)?;

        // Ok(profile)
        todo!()
    }
}

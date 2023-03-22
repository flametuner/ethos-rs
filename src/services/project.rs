use async_graphql::SimpleObject;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::{database::ConnectionPool, errors::StoreError, schema::projects};
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, SimpleObject)]
pub struct Project {
    id: Uuid,
    name: String,
    description: Option<String>,
    url: Option<String>,
    cors: Option<Vec<Option<String>>>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject {
    name: String,
    description: Option<String>,
    url: Option<String>,
    cors: Vec<String>,
}
pub struct ProjectService {
    pool: ConnectionPool,
}

impl ProjectService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        ProjectService {
            pool: ConnectionPool::new(pool),
        }
    }

    pub fn create_project(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<Project, StoreError> {
        let insert = NewProject {
            name,
            description,
            url: None,
            cors: vec![],
        };
        let mut conn = self.pool.get()?;
        diesel::insert_into(projects::table)
            .values(&insert)
            .get_result::<Project>(&mut conn)
            .map_err(|_e| StoreError::FailedToCreate)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, StoreError> {
        use crate::schema::projects::dsl::*;

        let mut conn = self.pool.get()?;
        projects
            .limit(10)
            .load::<Project>(&mut *conn)
            .map_err(|_| StoreError::LoadError)
    }
}

use async_graphql::SimpleObject;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

use crate::{database::ConnectionPool, errors::StoreError, schema::projects};
use diesel::{Insertable, PgConnection, Queryable};
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
    pub fn new(pool: ConnectionPool) -> Self {
        ProjectService { pool }
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
        let mut conn = self.get_conn()?;
        diesel::insert_into(projects::table)
            .values(&insert)
            .get_results::<Project>(&mut conn)
            .map(|mut p| p.pop().unwrap())
            .map_err(|_e| StoreError::FailedToCreate)
    }

    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, StoreError> {
        self.pool.get()
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, StoreError> {
        use crate::schema::projects::dsl::*;

        let mut conn = self.get_conn()?;
        projects
            .limit(10)
            .load::<Project>(&mut *conn)
            .map_err(|_| StoreError::LoadError)
    }
}

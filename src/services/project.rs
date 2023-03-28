use async_graphql::SimpleObject;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::{database::ConnectionPool, errors::StoreError, schema::projects};
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Debug, Queryable, SimpleObject, Identifiable)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: Uuid,
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
        name: &str,
        description: Option<String>,
    ) -> Result<Project, StoreError> {
        let insert = NewProject {
            name: name.to_string(),
            description,
            url: None,
            cors: vec![],
        };
        let mut conn = self.pool.get()?;
        Ok(diesel::insert_into(projects::table)
            .values(&insert)
            .get_result::<Project>(&mut conn)?)
    }

    pub fn get_project(&self, project: Uuid) -> Result<Project, StoreError> {
        use crate::schema::projects::dsl::*;
        let mut conn = self.pool.get()?;
        let project = projects.find(project).first(&mut conn)?;
        Ok(project)
    }

    pub fn get_project_by_name(&self, project_name: &str) -> Result<Project, StoreError> {
        use crate::schema::projects::dsl::*;
        let mut conn = self.pool.get()?;
        let project = projects.filter(name.eq(project_name)).first(&mut conn)?;
        Ok(project)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, StoreError> {
        use crate::schema::projects::dsl::*;

        let mut conn = self.pool.get()?;
        Ok(projects.limit(10).load::<Project>(&mut *conn)?)
    }
}

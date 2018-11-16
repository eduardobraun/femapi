use actix_web::{actix::Handler, error, Error};
use chrono::Utc;
use diesel::{self, sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::model::db::ConnDsl;
use crate::model::project::{CreateProject, NewProject, Project, ProjectById};
use crate::model::response::MyError;
use crate::share::common::Claims;

impl Handler<ProjectById> for ConnDsl {
    type Result = Result<Project, MyError>;

    fn handle(&mut self, project_by_id: ProjectById, _: &mut Self::Context) -> Self::Result {
        match Uuid::parse_str(&project_by_id.project_id) {
            Ok(id) => {
                use crate::share::schema::projects::dsl;
                let conn = &self.0.get().map_err(|_| MyError::DatabaseError)?;
                Ok(dsl::projects
                    .find(id)
                    .first::<Project>(conn)
                    .map_err(|_| MyError::NotFound)?)
            }
            Err(_e) => Err(MyError::NotFound),
        }
    }
}

impl Handler<CreateProject> for ConnDsl {
    type Result = Result<Project, MyError>;

    fn handle(&mut self, create_project: CreateProject, _: &mut Self::Context) -> Self::Result {
        use crate::share::schema::projects::dsl;
        let conn = &self.0.get().map_err(|_| MyError::DatabaseError)?;
        let new_project = NewProject {
            id: Uuid::new_v4(),
            name: &create_project.name,
            archived: false,
            created_at: Utc::now().naive_utc(),
        };

        let project: Project = diesel::insert_into(dsl::projects)
            .values(&new_project)
            .get_result(conn)
            .map_err(|_e| MyError::DatabaseError)?;
        Ok(project)
    }
}

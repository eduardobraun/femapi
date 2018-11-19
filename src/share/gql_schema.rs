use actix_web::actix::Addr;
use crate::model::db::ConnDsl;
use crate::model::{
    member::Member,
    project::{CreateProject, Project, ProjectById, ProjectMembers},
    user::{User, UserById, UserProjects},
};
use futures::Future;
use juniper::Context;
use juniper::RootNode;
use juniper::{FieldError, FieldResult};

pub struct SchemaContext {
    pub current_user: Option<User>,
    pub db_addr: Addr<ConnDsl>,
}

impl Context for SchemaContext {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

graphql_object!(User: SchemaContext |&self| {
    description: "An User"

    field id() -> String as "The unique id of the user" {
        self.id.hyphenated().to_string()
    }

    field name() -> &str as "The user username" {
        &self.username
    }

    field projects(&executor) -> FieldResult<Vec<Project>> {
        match executor
              .context()
              .db_addr
              .send(UserProjects{user: self.clone()})
              .wait()
              .unwrap() {
            Ok(projects) => Ok(projects),
            Err(_e) => Err(FieldError::new(
                "Could not get Project",
                graphql_value!({ "internal_error": ""})
            )),
        }
    }
});

graphql_object!(Project: SchemaContext |&self| {
    description: "A Project"

    field id() -> String as "The unique id of the project" {
        self.id.hyphenated().to_string()
    }

    field name() -> &str as "The project name" {
        &self.name
    }

    field members(&executor) -> FieldResult<Vec<Member>> as "Project members" {
        match executor
              .context()
              .db_addr
              .send(ProjectMembers{project: self.clone()})
              .wait()
              .unwrap() {
            Ok(members) => Ok(members),
            Err(_e) => Err(FieldError::new(
                "Could not get members for project",
                graphql_value!({ "internal_error": ""})
            )),
        }
    }

    // field files(&executor) -> Vec<FileNode> as "Project files" {
    //     vec![]
    // }
});

#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Permission {
    Read,
    Write,
    Owner,
}

graphql_object!(Member: SchemaContext |&self| {
    description: "A Member"

    field user(&executor) -> FieldResult<User> as "The user" {
        match executor
              .context()
              .db_addr
              .send(UserById{user_id: self.user_id.to_string()})
              .wait()
              .unwrap() {
            Ok(project) => Ok(project),
            Err(_e) => Err(FieldError::new(
                "Could not get User",
                graphql_value!({ "internal_error": ""})
            )),
        }
    }

    field project(&executor) -> FieldResult<Project> as "The project" {
        match executor
              .context()
              .db_addr
              .send(ProjectById{project_id: self.project_id.to_string()})
              .wait()
              .unwrap() {
            Ok(project) => Ok(project),
            Err(_e) => Err(FieldError::new(
                "Could not get Project",
                graphql_value!({ "internal_error": ""})
            )),
        }
    }

    field permission() -> Option<Permission> as "The project name" {
        if self.permission == "READ" {
            Some(Permission::Read)
        } else if self.permission == "WRITE" {
            Some(Permission::Write)
        } else if self.permission == "OWNER" {
            Some(Permission::Owner)
        } else {
            None
        }
    }
});

pub struct QueryRoot();
graphql_object!(QueryRoot: SchemaContext as "Query" |&self| {
    description: "The root query object of the schema"

    field current_user(&executor) -> FieldResult<User>
        as ""
    {
        match executor.context().current_user.clone() {
            Some(user) => Ok(user),
            None => Err(FieldError::new(
                "Not authenticated",
                graphql_value!({ "internal_error": "Could not parse the current user from the authentication token" })
            )),
        }
    }

    field users(&executor) -> Vec<User>
        as ""
    {
        vec![]
    }

    field projects(&executor) -> Vec<Project>
        as ""
    {
        vec![]
    }

    field project(&executor, id: String) -> FieldResult<Project>
        as ""
    {
        match executor
              .context()
              .db_addr
              .send(ProjectById{project_id: id})
              .wait()
              .unwrap() {
            Ok(project) => Ok(project),
            Err(_e) => Err(FieldError::new(
                "Could not get Project",
                graphql_value!({ "internal_error": ""})
            )),
        }
    }
});

pub struct MutationRoot;
graphql_object!(MutationRoot: SchemaContext as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field new_project(&executor, name: String) -> FieldResult<Project>
        as "Creates a new project"
    {
        match executor.context().current_user.clone() {
            Some(current_user) =>
            match executor
                .context()
                .db_addr
                .send(CreateProject{name: name, user: current_user})
                .wait()
                .unwrap() {
                Ok(project) => Ok(project),
                Err(_e) => Err(FieldError::new(
                    "Could not create Project",
                    graphql_value!({ "internal_error": ""})
                )),
            },
            None => Err(FieldError::new("Invalid credentials", graphql_value!({"internal_error": ""})))
        }
    }

    field delete_project(&executor, pid: String) -> bool as "Deletes a project" {
        false
    }
});

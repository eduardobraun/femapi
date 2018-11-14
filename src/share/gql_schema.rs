use crate::model::{project::Project, user::User};
use juniper::Context;
use juniper::FieldResult;
use juniper::RootNode;
use uuid::Uuid;

pub struct SchemaContext {
    pub current_user: Option<User>,
}

impl Context for SchemaContext {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Permission {
    Read,
    Write,
    Owner,
}

graphql_object!(User: SchemaContext |&self| {
    description: "An User"

    field id() -> String as "The unique id of the user" {
        self.id.hyphenated().to_string()
    }

    field name() -> &str as "The user username" {
        &self.username
    }

    field projects(&executor) -> Vec<Project> {
        vec![]
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

    // field members(&executor) -> Vec<Member> as "Project members" {
    //     vec![]
    // }

    // field files(&executor) -> Vec<FileNode> as "Project files" {
    //     vec![]
    // }
});

pub struct QueryRoot();
graphql_object!(QueryRoot: SchemaContext as "Query" |&self| {
    description: "The root query object of the schema"

    field current_user(&executor) -> Option<User>
        as ""
    {
        executor.context().current_user.clone()
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

    field project(&executor, id: String) -> Project
        as ""
    {
        Project::new()
    }
});

pub struct MutationRoot;
graphql_object!(MutationRoot: SchemaContext as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field new_project(&executor, name: String) -> Option<Project>
        as "Creates a new project"
    {
        None
    }

    field delete_project(&executor, pid: String) -> bool as "Deletes a project" {
        false
    }
});

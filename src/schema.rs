use super::db::models::{Member, NewMember, NewProject, Project, User};
use super::db::DbConn;
use super::db::Pool;
use super::diesel::prelude::*;

use juniper::Context;

pub struct Database {
    pub current_user: Option<User>,
    pub pool: Pool,
}

impl Context for Database {}

#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Permission {
    Read,
    Write,
    Owner,
}

graphql_object!(User: Database |&self| {
    description: "A todo item that that can be marked as completed"

    field id() -> i32 as "The unique id of the todo item" {
        self.id
    }

    field name() -> &str as "The user-editable title" {
        &self.name
    }

    field projects(&executor) -> Vec<Project> {
        use super::db::schema::projects::dsl as pdsl;
        use super::db::schema::members::dsl as mdsl;

        let connection = DbConn(executor.context().pool.get().unwrap());
        let members = mdsl::members.filter(mdsl::user_id.eq(self.id))
            .load::<Member>(&*connection)
            .expect("Failed to query users");

        let mut res: Vec<Project> = vec![];
        for m in members {
            let project = pdsl::projects.find(m.project_id).first::<Project>(&*connection).unwrap();
            res.push(project);
        }

        res
    }
});

graphql_object!(Project: Database |&self| {
    description: "A todo item that that can be marked as completed"

    field id() -> i32 as "The unique id of the todo item" {
        self.id
    }

    field name() -> &str as "The user-editable title" {
        &self.name
    }

    field members(&executor) -> Vec<Member> as "Project members" {
        use super::db::schema::members::dsl as mdsl;

        let connection = DbConn(executor.context().pool.get().unwrap());
        mdsl::members.filter(mdsl::project_id.eq(self.id))
            .load::<Member>(&*connection)
            .expect("Failed to query users")
    }
});

graphql_object!(Member: Database |&self| {
    description: "A todo item that that can be marked as completed"

    field project(&executor) -> Project as "The unique id of the todo item" {
        use super::db::schema::projects::dsl as pdsl;
        let connection = DbConn(executor.context().pool.get().unwrap());
        pdsl::projects.find(self.project_id).first::<Project>(&*connection).unwrap()
    }

    field user(&executor) -> User as "The unique id of the todo item" {
        use super::db::schema::users::dsl;
        let connection = DbConn(executor.context().pool.get().unwrap());
        dsl::users.find(self.user_id).first::<User>(&*connection).unwrap()
    }

    field permission() -> Option<Permission> as "The user-editable title" {
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
graphql_object!(QueryRoot: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field current_user(&executor) -> Option<User>
        as "Get all todo items in the system ordered by date"
    {
        executor.context().current_user.clone()
    }

    field users(&executor) -> Vec<User>
        as "Get all todo items in the system ordered by date"
    {
        use super::db::schema::users::dsl;

        let connection = DbConn(executor.context().pool.get().unwrap());
        dsl::users.order(dsl::id)
            .load::<User>(&*connection)
            .expect("Failed to query users")
    }

    field projects(&executor) -> Vec<Project>
        as "Get all todo items in the system ordered by date"
    {
        use super::db::schema::projects::dsl;

        let connection = DbConn(executor.context().pool.get().unwrap());
        dsl::projects.order(dsl::id)
            .load::<Project>(&*connection)
            .expect("Failed to query users")
    }

    field project(&executor, id: i32) -> Project
        as "Get all todo items in the system ordered by date"
    {
        use super::db::schema::projects::dsl;

        let connection = DbConn(executor.context().pool.get().unwrap());
        dsl::projects.find(id)
            .first::<Project>(&*connection)
            .expect("Failed to query users")
    }
});

pub struct MutationRoot;
graphql_object!(MutationRoot: Database as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field new_project(&executor, name: String) -> Option<Project>
        as "Creates a new project"
    {
        use super::db::schema::projects::dsl as pdsl;
        use super::db::schema::members::dsl as mdsl;

        let connection = DbConn(executor.context().pool.get().unwrap());

        let user = executor.context().current_user.clone().expect("No current user");
        let new_project = NewProject{name: &name, archived: false};
        diesel::insert_into(pdsl::projects)
        .values(&new_project)
        .execute(&*connection)
        .expect("Error saving new project");
        let project = pdsl::projects.order(pdsl::id.desc())
            .first::<Project>(&*connection)
            .expect("Failed to query project");
        let new_member = NewMember{user_id: user.id, project_id: project.id, permission: &"OWNER".to_owned()};
        diesel::insert_into(mdsl::members)
        .values(&new_member)
        .execute(&*connection)
        .expect("Error saving new member");
        println!{"{:?}", project};
        Some(project)
    }

    field delete_project(&executor, pid: i32) -> bool as "Deletes a project" {
        use super::db::schema::projects::dsl as pdsl;
        use super::db::schema::members::dsl as mdsl;
        let connection = DbConn(executor.context().pool.get().unwrap());
        diesel::delete(mdsl::members.filter(mdsl::project_id.eq(pid))).execute(&*connection).unwrap();
        diesel::delete(pdsl::projects.filter(pdsl::id.eq(pid))).execute(&*connection).unwrap();
        true
    }


});

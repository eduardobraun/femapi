use actix_web::actix::Handler;
use diesel::{self, RunQueryDsl};

use crate::model::db::ConnDsl;
use crate::model::member::{AddMember, Member};
use crate::model::response::MyError;

impl Handler<AddMember> for ConnDsl {
    type Result = Result<Member, MyError>;

    fn handle(&mut self, add_member: AddMember, _: &mut Self::Context) -> Self::Result {
        use crate::share::schema::members::dsl;
        let conn = &self.0.get().map_err(|_| MyError::DatabaseError)?;
        let new_member = Member {
            user_id: add_member.user.id.clone(),
            project_id: add_member.project.id.clone(),
            permission: add_member.permission,
        };

        let member: Member = diesel::insert_into(dsl::members)
            .values(&new_member)
            .get_result(conn)
            .map_err(|_e| MyError::DatabaseError)?;
        Ok(member)
    }
}

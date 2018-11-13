table! {
    members (user_id, project_id) {
        user_id -> Uuid,
        project_id -> Uuid,
        permission -> Varchar,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        name -> Varchar,
        archived -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(members -> projects (project_id));
joinable!(members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    members,
    projects,
    users,
);

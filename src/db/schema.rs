table! {
    members (user_id, project_id) {
        user_id -> Integer,
        project_id -> Integer,
        permission -> Text,
    }
}

table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        archived -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

joinable!(members -> projects (project_id));
joinable!(members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    members,
    projects,
    users,
);

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

allow_tables_to_appear_in_same_query!(
    projects,
    users,
);

table! {
    users (id) {
        created_at -> Timestamptz,
        email -> Varchar,
        first_name -> Varchar,
        id -> Int4,
        last_name -> Varchar,
        password_salt -> Text,
        updated_at -> Timestamptz,
    }
}

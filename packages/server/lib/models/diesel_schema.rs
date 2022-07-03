table! {
    users (id) {
        created_at -> Timestamptz,
        email -> Varchar,
        first_name -> Varchar,
        id -> Varchar,
        last_name -> Varchar,
        password_salt -> Text,
        updated_at -> Timestamptz,
    }
}

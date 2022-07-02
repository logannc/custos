table! {
    beats (id) {
        id -> Nullable<Integer>,
        quantity -> Integer,
        is_arcane -> Bool,
        player -> Text,
    }
}

table! {
    scenes (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    beats,
    scenes,
);

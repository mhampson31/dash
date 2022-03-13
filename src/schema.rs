table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    service (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        active -> Bool,
    }
}

table! {
    service_categories (service_id, category_id) {
        service_id -> Int4,
        category_id -> Int4,
    }
}

table! {
    user_categories (person_id, category_id) {
        person_id -> Int4,
        category_id -> Int4,
    }
}

joinable!(service_categories -> category (category_id));
joinable!(service_categories -> service (service_id));
joinable!(user_categories -> category (category_id));
joinable!(user_categories -> person (person_id));

allow_tables_to_appear_in_same_query!(
    category,
    person,
    service,
    service_categories,
    user_categories,
);

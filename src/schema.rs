table! {
    smor_chef_profiles (id) {
        id -> Int4,
        user_id -> Varchar,
        nickname -> Varchar,
        dish -> Varchar,
        dish_cost -> Varchar,
        details -> Text,
        rating -> Int4,
        icon -> Varchar,
        experience -> Varchar,
        state -> Varchar,
        lga -> Varchar,
        verification_status -> Bool,
        availability_status -> Bool,
        next_of_kin_full_name -> Varchar,
        next_of_kin_address -> Varchar,
        next_of_kin_phone -> Varchar,
        next_of_kin_relationship -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    smor_users (id) {
        id -> Int4,
        user_id -> Varchar,
        name -> Varchar,
        phone -> Varchar,
        avatar -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Int4,
        status -> Bool,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    smor_chef_profiles,
    smor_users,
);

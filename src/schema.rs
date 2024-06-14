// @generated automatically by Diesel CLI.

diesel::table! {
    dish (dish_id) {
        dish_id -> Int4,
        price -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        dish_type -> Varchar,
    }
}

diesel::table! {
    dish_tag (dish_tag_id) {
        dish_tag_id -> Int4,
        id_dish -> Int4,
        id_tag -> Int4,
    }
}

diesel::table! {
    menucard (menucard_id) {
        menucard_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    menucard_dish (menucard_dish_id) {
        menucard_dish_id -> Int4,
        id_menucard -> Int4,
        id_dish -> Int4,
        chefs_choice -> Bool,
    }
}

diesel::table! {
    person (person_id) {
        person_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
    }
}

diesel::table! {
    reservation (reservation_id) {
        reservation_id -> Int4,
        id_person -> Int4,
        end_timestamp -> Timestamp,
        start_timestamp -> Timestamp,
        person_count -> Int4,
    }
}

diesel::table! {
    setting (setting_id) {
        setting_id -> Int4,
        id_menucard_active -> Int4,
        restaurant_width -> Int4,
        restaurant_height -> Int4,
    }
}

diesel::table! {
    table (table_id) {
        table_id -> Int4,
        seat_count -> Int4,
        coord_x -> Int4,
        coord_y -> Int4,
        width -> Int4,
        height -> Int4,
    }
}

diesel::table! {
    table_reservation (table_reservation_id) {
        table_reservation_id -> Int4,
        id_table -> Int4,
        id_reservation -> Int4,
    }
}

diesel::table! {
    tag (tag_id) {
        tag_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(dish_tag -> dish (id_dish));
diesel::joinable!(dish_tag -> tag (id_tag));
diesel::joinable!(menucard_dish -> dish (id_dish));
diesel::joinable!(menucard_dish -> menucard (id_menucard));
diesel::joinable!(reservation -> person (id_person));
diesel::joinable!(setting -> menucard (id_menucard_active));
diesel::joinable!(table_reservation -> reservation (id_reservation));
diesel::joinable!(table_reservation -> table (id_table));

diesel::allow_tables_to_appear_in_same_query!(
    dish,
    dish_tag,
    menucard,
    menucard_dish,
    person,
    reservation,
    setting,
    table,
    table_reservation,
    tag,
);

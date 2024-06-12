// @generated automatically by Diesel CLI.

diesel::table! {
    dish (dish_id) {
        dish_id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    dish_tag (dish_tag_id) {
        dish_tag_id -> Nullable<Integer>,
        id_dish -> Nullable<Integer>,
        id_tag -> Nullable<Integer>,
    }
}

diesel::table! {
    menu_card (menucard_id) {
        menucard_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    menu_card_dish (menucard_dish_id) {
        menucard_dish_id -> Nullable<Integer>,
        id_menucard -> Nullable<Integer>,
        id_dish -> Nullable<Integer>,
        chefs_choice -> Nullable<Bool>,
    }
}

diesel::table! {
    person (person_id) {
        person_id -> Nullable<Integer>,
        name -> Text,
        phone -> Text,
    }
}

diesel::table! {
    reservation (reservation_id) {
        reservation_id -> Nullable<Integer>,
        id_table -> Nullable<Integer>,
        id_person -> Nullable<Integer>,
        start_timestamp -> Nullable<Text>,
        end_timestamp -> Nullable<Text>,
    }
}

diesel::table! {
    restaurant_table (table_id) {
        table_id -> Nullable<Integer>,
        seat_count -> Nullable<Integer>,
        coord_x -> Nullable<Integer>,
        coord_y -> Nullable<Integer>,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
    }
}

diesel::table! {
    setting (setting_id) {
        setting_id -> Nullable<Integer>,
        id_menucard_active -> Nullable<Integer>,
        restaurant_width -> Nullable<Integer>,
        restaurant_height -> Nullable<Integer>,
    }
}

diesel::table! {
    table_reservation (table_reservation_id) {
        table_reservation_id -> Nullable<Integer>,
        id_table -> Nullable<Integer>,
        id_reservation -> Nullable<Integer>,
    }
}

diesel::table! {
    tag (tag_id) {
        tag_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::joinable!(dish_tag -> dish (id_dish));
diesel::joinable!(dish_tag -> tag (id_tag));
diesel::joinable!(menu_card_dish -> dish (id_dish));
diesel::joinable!(menu_card_dish -> menu_card (id_menucard));
diesel::joinable!(reservation -> person (id_person));
diesel::joinable!(reservation -> restaurant_table (id_table));
diesel::joinable!(setting -> menu_card (id_menucard_active));
diesel::joinable!(table_reservation -> reservation (id_reservation));
diesel::joinable!(table_reservation -> restaurant_table (id_table));

diesel::allow_tables_to_appear_in_same_query!(
    dish,
    dish_tag,
    menu_card,
    menu_card_dish,
    person,
    reservation,
    restaurant_table,
    setting,
    table_reservation,
    tag,
);

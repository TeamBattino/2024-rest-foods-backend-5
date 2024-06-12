-- Enable foreign key support
PRAGMA foreign_keys = ON;
-- Create Table 'tag'
CREATE TABLE tag (
    tag_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
-- Create Table 'dish'
CREATE TABLE dish (
    dish_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT NOT NULL
);
-- Create Table 'dish_tag'
CREATE TABLE dish_tag (
    dish_tag_id INTEGER PRIMARY KEY,
    id_dish INTEGER,
    id_tag INTEGER,
    FOREIGN KEY (id_dish) REFERENCES dish(dish_id),
    FOREIGN KEY (id_tag) REFERENCES tag(tag_id)
);
-- Create Table 'menu_card'
CREATE TABLE menu_card (
    menucard_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
-- Create Table 'menu_card_dish'
CREATE TABLE menu_card_dish (
    menucard_dish_id INTEGER PRIMARY KEY,
    id_menucard INTEGER,
    id_dish INTEGER,
    chefs_choice BOOLEAN,
    FOREIGN KEY (id_menucard) REFERENCES menu_card(menucard_id),
    FOREIGN KEY (id_dish) REFERENCES dish(dish_id)
);
-- Create Table 'setting'
CREATE TABLE setting (
    setting_id INTEGER PRIMARY KEY,
    id_menucard_active INTEGER,
    restaurant_width INTEGER,
    restaurant_height INTEGER,
    FOREIGN KEY (id_menucard_active) REFERENCES menu_card(menucard_id)
);
-- Create Table 'restaurant_table'
CREATE TABLE restaurant_table (
    table_id INTEGER PRIMARY KEY,
    seat_count INTEGER,
    coord_x INTEGER,
    coord_y INTEGER,
    width INTEGER,
    height INTEGER
);
-- Create Table 'person'
CREATE TABLE person (
    person_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    phone TEXT NOT NULL
);
-- Create Table 'reservation'
CREATE TABLE reservation (
    reservation_id INTEGER PRIMARY KEY,
    id_table INTEGER,
    id_person INTEGER,
    start_timestamp TEXT,
    end_timestamp TEXT,
    FOREIGN KEY (id_table) REFERENCES restaurant_table(table_id),
    FOREIGN KEY (id_person) REFERENCES person(person_id)
);
-- Create Table 'table_reservation'
CREATE TABLE table_reservation (
    table_reservation_id INTEGER PRIMARY KEY,
    id_table INTEGER,
    id_reservation INTEGER,
    FOREIGN KEY (id_table) REFERENCES restaurant_table(table_id),
    FOREIGN KEY (id_reservation) REFERENCES reservation(reservation_id)
);
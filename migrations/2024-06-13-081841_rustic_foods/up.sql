-- Your SQL goes here
CREATE TABLE setting (
    setting_id SERIAL PRIMARY KEY,
    id_menucard_active INT NOT NULL,
    restaurant_width INT NOT NULL,
    restaurant_height INT NOT NULL
);
CREATE TABLE menucard (
    menucard_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);
CREATE TABLE menucard_dish(
    menucard_dish_id SERIAL PRIMARY KEY,
    id_menucard INT NOT NULL,
    id_dish INT NOT NULL,
    chefs_choice BOOLEAN NOT NULL
);
CREATE TABLE dish(
    dish_id SERIAL PRIMARY KEY,
    price INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    dish_type VARCHAR(255) NOT NULL
);
CREATE TABLE dish_tag(
    dish_tag_id SERIAL PRIMARY KEY,
    id_dish INT NOT NULL,
    id_tag INT NOT NULL
);
CREATE TABLE tag(
    tag_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);
ALTER TABLE dish_tag
ADD FOREIGN KEY (id_dish) REFERENCES dish(dish_id);
ALTER TABLE dish_tag
ADD FOREIGN KEY (id_tag) REFERENCES tag(tag_id);
ALTER TABLE menucard_dish
ADD FOREIGN KEY (id_menucard) REFERENCES menucard(menucard_id);
ALTER TABLE menucard_dish
ADD FOREIGN KEY (id_dish) REFERENCES dish(dish_id);
ALTER TABLE setting
ADD FOREIGN KEY (id_menucard_active) REFERENCES menucard(menucard_id);
CREATE TABLE "table" (
    table_id SERIAL PRIMARY KEY,
    seat_count INT NOT NULL,
    coord_x INT NOT NULL,
    coord_y INT NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL
);
CREATE TABLE table_reservation(
    table_reservation_id SERIAL PRIMARY KEY,
    id_table INT NOT NULL,
    id_reservation INT NOT NULL
);
CREATE TABLE reservation(
    reservation_id SERIAL PRIMARY KEY,
    id_person INT NOT NULL,
    end_timestamp TIMESTAMP NOT NULL,
    start_timestamp TIMESTAMP NOT NULL,
    person_count INT NOT NULL
);
CREATE TABLE person(
    person_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL
);
ALTER TABLE table_reservation
ADD FOREIGN KEY (id_table) REFERENCES "table"(table_id);
ALTER TABLE table_reservation
ADD FOREIGN KEY (id_reservation) REFERENCES reservation(reservation_id);
ALTER TABLE reservation
ADD FOREIGN KEY (id_person) REFERENCES person(person_id);
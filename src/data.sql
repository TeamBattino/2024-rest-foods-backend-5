-- Insert data into menucard table
INSERT INTO menucard (name)
VALUES ('Summer Specials'),
    ('Winter Warmers');
-- Insert data into dish table
INSERT INTO dish (price ,name, description, dish_type)
VALUES (
        1000,
        'Spaghetti Carbonara',
        'A classic Italian pasta dish made with eggs, cheese, pancetta, and pepper.',
        'Main Course'

    ),
    (
        999,
        'Caesar Salad',
        'A salad of romaine lettuce and croutons dressed with lemon juice, olive oil, egg, Worcestershire sauce, anchovies, garlic, Dijon mustard, Parmesan cheese, and black pepper.',
        'Appetizer'

    );

-- Insert data into tag table
INSERT INTO tag (name)
VALUES ('Popular'),
    ('Vegetarian');

-- Insert data into person table
INSERT INTO person (name, phone)
VALUES ('John Doe', '123-456-7890'),
    ('Jane Smith', '987-654-3210'),
    ('Alice Johnson', '555-123-4567');
-- Insert data into table
INSERT INTO "table" (seat_count, coord_x, coord_y, width, height)
VALUES (4, 10, 15, 5, 5),
    (2, 20, 25, 3, 3);
-- Insert data into reservation table
INSERT INTO reservation (
        id_table,
        id_person,
        start_timestamp,
        end_timestamp,
        person_count
    )
VALUES (
        1,
        1,
        '2024-06-12T18:00:00Z',
        '2024-06-12T20:00:00Z',
        4
    ),
    (
        2,
        2,
        '2024-06-12T19:00:00Z',
        '2024-06-12T21:00:00Z',
        2
    );
-- Insert data into table_reservation table
INSERT INTO table_reservation (id_table, id_reservation)
VALUES (1, 1),
    (2, 2);
-- Insert data into setting table
INSERT INTO setting (
        id_menucard_active,
        restaurant_width,
        restaurant_height
    )
VALUES (1, 50, 30);

-- Insert data into menucard_dish table
INSERT INTO menucard_dish (id_menucard, id_dish, chefs_choice)
VALUES (1, 1, true),
       (1, 2, false);

-- Insert data into dish_tag table
INSERT INTO dish_tag (id_dish, id_tag)
VALUES (1, 1),
       (2, 2);
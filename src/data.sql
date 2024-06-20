-- Insert sample menucards with explicit IDs
INSERT INTO menucard (menucard_id, name)
VALUES (1, 'Breakfast Menu'),
    (2, 'Lunch Menu'),
    (3, 'Dinner Menu');
-- Insert sample settings
INSERT INTO setting (
        setting_id,
        id_menucard_active,
        restaurant_width,
        restaurant_height
    )
VALUES (1, 1, 500, 300);
-- Insert sample dishes with explicit IDs
INSERT INTO dish (dish_id, price, name, description, dish_type)
VALUES (
        1,
        1000,
        'Pancakes',
        'Fluffy pancakes with syrup',
        'Breakfast'
    ),
    (
        2,
        1200,
        'Omelette',
        'Cheese omelette with mushrooms',
        'Breakfast'
    ),
    (
        3,
        1500,
        'Burger',
        'Beef burger with fries',
        'Lunch'
    ),
    (
        4,
        1700,
        'Salad',
        'Caesar salad with chicken',
        'Lunch'
    ),
    (
        5,
        2500,
        'Steak',
        'Grilled steak with vegetables',
        'Dinner'
    );
-- Insert sample menucard dishes with explicit IDs
INSERT INTO menucard_dish (
        menucard_dish_id,
        id_menucard,
        id_dish,
        chefs_choice
    )
VALUES (1, 1, 1, TRUE),
    -- Pancakes on Breakfast Menu
    (2, 1, 2, FALSE),
    -- Omelette on Breakfast Menu
    (3, 2, 3, TRUE),
    -- Burger on Lunch Menu
    (4, 2, 4, FALSE),
    -- Salad on Lunch Menu
    (5, 3, 5, TRUE);
-- Steak on Dinner Menu
-- Insert sample tags with explicit IDs
INSERT INTO tag (tag_id, name)
VALUES (1, 'Gluten-Free'),
    (2, 'Vegan'),
    (3, 'Spicy'),
    (4, 'Popular');
-- Insert sample dish tags with explicit IDs
INSERT INTO dish_tag (dish_tag_id, id_dish, id_tag)
VALUES (1, 1, 4),
    -- Pancakes are popular
    (2, 2, 2),
    -- Omelette is vegan
    (3, 3, 3),
    -- Burger is spicy
    (4, 4, 4),
    -- Salad is popular
    (5, 5, 4);
-- Steak is popular
-- Insert sample tables with explicit IDs
INSERT INTO "table" (
        table_id,
        seat_count,
        coord_x,
        coord_y,
        width,
        height
    )
VALUES (1, 4, 10, 20, 100, 50),
    (2, 2, 50, 100, 60, 60),
    (3, 6, 150, 200, 120, 80);
-- Insert sample persons with explicit IDs
INSERT INTO person (person_id, name, phone)
VALUES (1, 'John Doe', '555-1234'),
    (2, 'Jane Smith', '555-5678'),
    (3, 'Alice Johnson', '555-9101');
-- Insert sample reservations with explicit IDs
INSERT INTO reservation (
        reservation_id,
        id_person,
        start_timestamp,
        end_timestamp,
        person_count
    )
VALUES (
        1,
        1,
        '2024-06-20 18:00:00',
        '2024-06-20 20:00:00',
        4
    ),
    (
        2,
        2,
        '2024-06-21 12:00:00',
        '2024-06-21 14:00:00',
        2
    ),
    (
        3,
        3,
        '2024-06-22 19:00:00',
        '2024-06-22 21:00:00',
        6
    );
-- Insert sample table reservations with explicit IDs
INSERT INTO table_reservation (table_reservation_id, id_table, id_reservation)
VALUES (1, 1, 1),
    (2, 2, 2),
    (3, 3, 3);
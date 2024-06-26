-- Insert sample menucards with explicit IDs
INSERT INTO menucard (menucard_id, name)
VALUES (4, 'Rusty Breakfast Menu'),
    (5, 'Rusty Lunch Menu'),
    (6, 'Rusty Dinner Menu');
-- Insert sample settings
INSERT INTO setting (
        setting_id,
        id_menucard_active,
        restaurant_width,
        restaurant_height
    )
VALUES (2, 4, 600, 400);
-- Insert sample dishes with explicit IDs
INSERT INTO dish (dish_id, price, name, description, dish_type)
VALUES -- Breakfast Dishes
    (
        6,
        800,
        'Rusty Pancakes',
        'Pancakes with a hint of iron',
        'Breakfast'
    ),
    (
        7,
        950,
        'Rusty Omelette',
        'Omelette with rusty cheese',
        'Breakfast'
    ),
    (
        11,
        1000,
        'Rusty Bacon',
        'Crispy bacon with a rusty edge',
        'Breakfast'
    ),
    (
        12,
        850,
        'Rusty French Toast',
        'French toast with rusty syrup',
        'Breakfast'
    ),
    (
        13,
        900,
        'Rusty Waffles',
        'Waffles with a rusty crunch',
        'Breakfast'
    ),
    (
        14,
        950,
        'Rusty Cereal',
        'Cereal flakes with rust powder',
        'Breakfast'
    ),
    (
        15,
        1000,
        'Rusty Smoothie',
        'Smoothie with a rusty twist',
        'Breakfast'
    ),
    (
        16,
        1100,
        'Rusty Sausage',
        'Sausages with a rusty flavor',
        'Breakfast'
    ),
    (
        17,
        1200,
        'Rusty Hash Browns',
        'Crispy hash browns with rust',
        'Breakfast'
    ),
    (
        18,
        1300,
        'Rusty Eggs Benedict',
        'Eggs Benedict with rusty hollandaise',
        'Breakfast'
    ),
    -- Lunch Dishes
    (
        8,
        1100,
        'Rusty Burger',
        'Burger with a rusty patty',
        'Lunch'
    ),
    (
        9,
        1200,
        'Rusty Salad',
        'Salad with rusty croutons',
        'Lunch'
    ),
    (
        19,
        1300,
        'Rusty Sandwich',
        'Sandwich with rusty fillings',
        'Lunch'
    ),
    (
        20,
        1400,
        'Rusty Soup',
        'Soup with rusty broth',
        'Lunch'
    ),
    (
        21,
        1500,
        'Rusty Tacos',
        'Tacos with rusty beef',
        'Lunch'
    ),
    (
        22,
        1600,
        'Rusty Pizza',
        'Pizza with rusty toppings',
        'Lunch'
    ),
    (
        23,
        1700,
        'Rusty Pasta',
        'Pasta with rusty sauce',
        'Lunch'
    ),
    (
        24,
        1800,
        'Rusty Chicken Wrap',
        'Wrap with rusty chicken',
        'Lunch'
    ),
    (
        25,
        1900,
        'Rusty Quesadilla',
        'Quesadilla with a rusty twist',
        'Lunch'
    ),
    (
        26,
        2000,
        'Rusty Mac and Cheese',
        'Mac and cheese with rusty flavor',
        'Lunch'
    ),
    -- Dinner Dishes
    (
        10,
        1800,
        'Rusty Steak',
        'Steak with a rusty glaze',
        'Dinner'
    ),
    (
        27,
        1900,
        'Rusty Salmon',
        'Salmon with rusty seasoning',
        'Dinner'
    ),
    (
        28,
        2000,
        'Rusty Lobster',
        'Lobster with a rusty finish',
        'Dinner'
    ),
    (
        29,
        2100,
        'Rusty Pork Chops',
        'Pork chops with rusty marinade',
        'Dinner'
    ),
    (
        30,
        2200,
        'Rusty Lamb Chops',
        'Lamb chops with rusty herbs',
        'Dinner'
    ),
    (
        31,
        2300,
        'Rusty Beef Wellington',
        'Beef Wellington with rusty pastry',
        'Dinner'
    ),
    (
        32,
        2400,
        'Rusty Shrimp',
        'Shrimp with rusty garlic sauce',
        'Dinner'
    ),
    (
        33,
        2500,
        'Rusty Duck',
        'Duck with rusty plum sauce',
        'Dinner'
    ),
    (
        34,
        2600,
        'Rusty Ribs',
        'Ribs with rusty BBQ sauce',
        'Dinner'
    ),
    (
        35,
        2700,
        'Rusty Meatloaf',
        'Meatloaf with rusty ketchup glaze',
        'Dinner'
    );
-- Insert sample menucard dishes with explicit IDs
INSERT INTO menucard_dish (
        menucard_dish_id,
        id_menucard,
        id_dish,
        chefs_choice
    )
VALUES -- Rusty Breakfast Menu
    (6, 4, 6, TRUE),
    -- Rusty Pancakes
    (7, 4, 7, FALSE),
    -- Rusty Omelette
    (11, 4, 11, FALSE),
    -- Rusty Bacon
    (12, 4, 12, FALSE),
    -- Rusty French Toast
    (13, 4, 13, TRUE),
    -- Rusty Waffles
    (14, 4, 14, FALSE),
    -- Rusty Cereal
    (15, 4, 15, FALSE),
    -- Rusty Smoothie
    (16, 4, 16, FALSE),
    -- Rusty Sausage
    (17, 4, 17, FALSE),
    -- Rusty Hash Browns
    (18, 4, 18, TRUE),
    -- Rusty Eggs Benedict
    -- Rusty Lunch Menu
    (8, 5, 8, TRUE),
    -- Rusty Burger
    (9, 5, 9, FALSE),
    -- Rusty Salad
    (19, 5, 19, FALSE),
    -- Rusty Sandwich
    (20, 5, 20, FALSE),
    -- Rusty Soup
    (21, 5, 21, TRUE),
    -- Rusty Tacos
    (22, 5, 22, FALSE),
    -- Rusty Pizza
    (23, 5, 23, FALSE),
    -- Rusty Pasta
    (24, 5, 24, FALSE),
    -- Rusty Chicken Wrap
    (25, 5, 25, FALSE),
    -- Rusty Quesadilla
    (26, 5, 26, TRUE),
    -- Rusty Mac and Cheese
    -- Rusty Dinner Menu
    (10, 6, 10, TRUE),
    -- Rusty Steak
    (27, 6, 27, FALSE),
    -- Rusty Salmon
    (28, 6, 28, FALSE),
    -- Rusty Lobster
    (29, 6, 29, TRUE),
    -- Rusty Pork Chops
    (30, 6, 30, FALSE),
    -- Rusty Lamb Chops
    (31, 6, 31, FALSE),
    -- Rusty Beef Wellington
    (32, 6, 32, FALSE),
    -- Rusty Shrimp
    (33, 6, 33, FALSE),
    -- Rusty Duck
    (34, 6, 34, FALSE),
    -- Rusty Ribs
    (35, 6, 35, TRUE);
-- Rusty Meatloaf
-- Insert sample tags with explicit IDs
INSERT INTO tag (tag_id, name)
VALUES (5, 'Rusty'),
    (6, 'Iron-Rich'),
    (7, 'Unique'),
    (8, 'Chef\'s Special'),
    (9, 'New');
-- Insert sample dish tags with explicit IDs
INSERT INTO dish_tag (dish_tag_id, id_dish, id_tag)
VALUES -- Rusty dishes
    (6, 6, 5),
    (7, 7, 5),
    (8, 8, 5),
    (9, 9, 5),
    (10, 10, 5),
    (11, 11, 5),
    (12, 12, 5),
    (13, 13, 5),
    (14, 14, 5),
    (15, 15, 5),
    (16, 16, 5),
    (17, 17, 5),
    (18, 18, 5),
    (19, 19, 5),
    (20, 20, 5),
    (21, 21, 5),
    (22, 22, 5),
    (23, 23, 5),
    (24, 24, 5),
    (25, 25, 5),
    (26, 26, 5),
    (27, 27, 5),
    (28, 28, 5),
    (29, 29, 5),
    (30, 30, 5),
    (31, 31, 5),
    (32, 32, 5),
    (33, 33, 5),
    (34, 34, 5),
    (35, 35, 5),
    -- Iron-Rich dishes
    (36, 6, 6),
    (37, 7, 6),
    (38, 10, 6),
    (39, 11, 6),
    (40, 15, 6),
    (41, 16, 6),
    (42, 27, 6),
    (43, 28, 6),
    (44, 30, 6),
    (45, 32, 6),
    -- Unique dishes
    (46, 6, 7),
    (47, 10, 7),
    (48, 13, 7),
    (49, 18, 7),
    (50, 21, 7),
    (51, 26, 7),
    (52, 29, 7),
    (53, 31, 7),
    (54, 34, 7),
    (55, 35, 7),
    -- Chef's Special dishes
    (56, 6, 8),
    (57, 10, 8),
    (58, 13, 8),
    (59, 18, 8),
    (60, 21, 8),
    (61, 26, 8),
    (62, 29, 8),
    (63, 31, 8),
    (64, 34, 8),
    (65, 35, 8),
    -- New dishes
    (66, 11, 9),
    (67, 12, 9),
    (68, 14, 9),
    (69, 15, 9),
    (70, 17, 9),
    (71, 19, 9),
    (72, 20, 9),
    (73, 22, 9),
    (74, 23, 9),
    (75, 25, 9);
-- Insert sample tables with explicit IDs
INSERT INTO "table" (
        table_id,
        seat_count,
        coord_x,
        coord_y,
        width,
        height
    )
VALUES (4, 4, 30, 40, 110, 55),
    (5, 2, 70, 120, 65, 65),
    (6, 6, 160, 210, 130, 85);
-- Insert sample persons with explicit IDs
INSERT INTO person (person_id, name, phone)
VALUES (4, 'Rusty Jones', '555-2024'),
    (5, 'Rustina Smith', '555-3035'),
    (6, 'Irona Brown', '555-4046');
-- Insert sample reservations with explicit IDs
INSERT INTO reservation (
        reservation_id,
        id_person,
        start_timestamp,
        end_timestamp,
        person_count
    )
VALUES (
        4,
        4,
        '2024-06-23 18:00:00',
        '2024-06-23 20:00:00',
        4
    ),
    (
        5,
        5,
        '2024-06-24 12:00:00',
        '2024-06-24 14:00:00',
        2
    ),
    (
        6,
        6,
        '2024-06-25 19:00:00',
        '2024-06-25 21:00:00',
        6
    );
-- Insert sample table reservations with explicit IDs
INSERT INTO table_reservation (table_reservation_id, id_table, id_reservation)
VALUES (4, 4, 4),
    (5, 5, 5),
    (6, 6, 6);
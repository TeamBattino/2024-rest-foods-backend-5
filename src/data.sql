ALTER SEQUENCE menucard_menucard_id_seq RESTART WITH 4;
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
        'Pancakes made with a special batter infused with a hint of iron, topped with syrup that has a unique rusty flavor.',
        'Breakfast'
    ),
    (
        7,
        950,
        'Rusty Omelette',
        'A fluffy omelette filled with cheese that has been aged to develop a distinct rusty taste, combined with sautéed rusty mushrooms.',
        'Breakfast'
    ),
    (
        11,
        1000,
        'Rusty Bacon',
        'Crispy bacon strips cooked to perfection with a rusty edge, providing a unique crunch and flavor.',
        'Breakfast'
    ),
    (
        12,
        850,
        'Rusty French Toast',
        'French toast slices soaked in a rusty egg mixture and fried golden brown, served with rusty syrup.',
        'Breakfast'
    ),
    (
        13,
        900,
        'Rusty Waffles',
        'Waffles with a rusty crunch, served with a side of rusty butter and syrup, creating a delightful breakfast experience.',
        'Breakfast'
    ),
    (
        14,
        950,
        'Rusty Cereal',
        'A bowl of cereal flakes dusted with rust powder, providing an iron-rich and crunchy breakfast.',
        'Breakfast'
    ),
    (
        15,
        1000,
        'Rusty Smoothie',
        'A smoothie blend of fruits and vegetables with a rusty twist, packed with nutrients and a unique flavor.',
        'Breakfast'
    ),
    (
        16,
        1100,
        'Rusty Sausage',
        'Sausages infused with a rusty flavor, offering a savory and iron-rich breakfast option.',
        'Breakfast'
    ),
    (
        17,
        1200,
        'Rusty Hash Browns',
        'Crispy hash browns with a hint of rust, providing a satisfying crunch and unique taste.',
        'Breakfast'
    ),
    (
        18,
        1300,
        'Rusty Eggs Benedict',
        'Classic Eggs Benedict topped with a rusty hollandaise sauce, offering a gourmet breakfast experience.',
        'Breakfast'
    ),
    (
        36,
        1400,
        'Rusty Sunrise Parfait',
        'A delicious yogurt parfait layered with rusty granola and fresh fruits, perfect for a healthy and unique breakfast.',
        'Breakfast'
    ),
    (
        37,
        1500,
        'Rusty Sunrise Skillet',
        'A hearty breakfast skillet with eggs, potatoes, and rusty seasoning, perfect for starting the day right.',
        'Breakfast'
    ),
    -- Lunch Dishes
    (
        8,
        1100,
        'Rusty Burger',
        'A juicy burger with a patty that has been marinated in a rusty sauce, topped with cheese, lettuce, and tomato.',
        'Lunch'
    ),
    (
        9,
        1200,
        'Rusty Salad',
        'A fresh salad with mixed greens, rusty croutons, and a dressing that has a subtle rusty flavor.',
        'Lunch'
    ),
    (
        19,
        1300,
        'Rusty Sandwich',
        'A sandwich filled with meats and cheeses that have been seasoned with rust, offering a unique twist on a classic.',
        'Lunch'
    ),
    (
        20,
        1400,
        'Rusty Soup',
        'A warm and comforting soup with a broth that has been infused with rusty spices, perfect for any meal.',
        'Lunch'
    ),
    (
        21,
        1500,
        'Rusty Tacos',
        'Tacos filled with beef that has been seasoned with rusty spices, topped with lettuce, cheese, and a rusty salsa.',
        'Lunch'
    ),
    (
        22,
        1600,
        'Rusty Pizza',
        'A pizza topped with rusty vegetables and meats, baked to perfection with a crispy rusty crust.',
        'Lunch'
    ),
    (
        23,
        1700,
        'Rusty Pasta',
        'Pasta tossed in a rusty sauce, providing a rich and flavorful lunch option.',
        'Lunch'
    ),
    (
        24,
        1800,
        'Rusty Chicken Wrap',
        'A wrap filled with chicken that has been marinated in a rusty sauce, along with fresh vegetables and cheese.',
        'Lunch'
    ),
    (
        25,
        1900,
        'Rusty Quesadilla',
        'A quesadilla filled with cheese and rusty chicken, offering a crispy and savory lunch option.',
        'Lunch'
    ),
    (
        26,
        2000,
        'Rusty Mac and Cheese',
        'Classic mac and cheese with a rusty twist, providing a rich and creamy lunch option.',
        'Lunch'
    ),
    (
        38,
        2100,
        'Rusty Reuben',
        'A Reuben sandwich with corned beef that has been aged to develop a rusty flavor, topped with sauerkraut and Swiss cheese.',
        'Lunch'
    ),
    (
        39,
        2200,
        'Rusty Cobb Salad',
        'A Cobb salad with a variety of fresh ingredients and a dressing that has a subtle rusty flavor.',
        'Lunch'
    ),
    (
        40,
        2300,
        'Rusty Flatbread',
        'Flatbread topped with rusty vegetables and meats, offering a unique and delicious lunch option.',
        'Lunch'
    ),
    (
        41,
        2400,
        'Rusty Sliders',
        'Mini burgers with patties that have been seasoned with rust, served with a side of rusty fries.',
        'Lunch'
    ),
    (
        42,
        2500,
        'Rusty BLT',
        'A BLT sandwich with bacon that has been cooked to develop a rusty edge, along with fresh lettuce and tomato.',
        'Lunch'
    ),
    -- Dinner Dishes
    (
        10,
        1800,
        'Rusty Steak',
        'A tender steak glazed with a rusty sauce, providing a rich and flavorful dinner option.',
        'Dinner'
    ),
    (
        27,
        1900,
        'Rusty Salmon',
        'Salmon fillets seasoned with rusty spices and grilled to perfection, offering a healthy and delicious dinner.',
        'Dinner'
    ),
    (
        28,
        2000,
        'Rusty Lobster',
        'Lobster tails finished with a rusty glaze, providing a gourmet seafood dinner experience.',
        'Dinner'
    ),
    (
        29,
        2100,
        'Rusty Pork Chops',
        'Pork chops marinated in a rusty marinade, grilled to perfection and served with a side of rusty potatoes.',
        'Dinner'
    ),
    (
        30,
        2200,
        'Rusty Lamb Chops',
        'Lamb chops seasoned with rusty herbs, grilled to perfection and served with a side of rusty vegetables.',
        'Dinner'
    ),
    (
        31,
        2300,
        'Rusty Beef Wellington',
        'Beef Wellington with a pastry crust that has a hint of rust, providing a rich and gourmet dinner option.',
        'Dinner'
    ),
    (
        32,
        2400,
        'Rusty Shrimp',
        'Shrimp tossed in a garlic sauce that has a subtle rusty flavor, served with a side of rusty rice.',
        'Dinner'
    ),
    (
        33,
        2500,
        'Rusty Duck',
        'Duck breast glazed with a rusty plum sauce, providing a rich and flavorful dinner experience.',
        'Dinner'
    ),
    (
        34,
        2600,
        'Rusty Ribs',
        'Ribs cooked to perfection with a rusty BBQ sauce, providing a savory and satisfying dinner.',
        'Dinner'
    ),
    (
        35,
        2700,
        'Rusty Meatloaf',
        'Meatloaf topped with a rusty ketchup glaze, providing a rich and comforting dinner option.',
        'Dinner'
    ),
    (
        43,
        2800,
        'Rusty Prime Rib',
        'Prime rib with a crust that has been seasoned with rusty spices, providing a rich and flavorful dinner.',
        'Dinner'
    ),
    (
        44,
        2900,
        'Rusty Pot Roast',
        'Pot roast cooked with rusty vegetables, providing a hearty and comforting dinner option.',
        'Dinner'
    ),
    (
        45,
        3000,
        'Rusty Stuffed Peppers',
        'Peppers stuffed with a rusty filling, providing a rich and flavorful dinner option.',
        'Dinner'
    ),
    (
        46,
        3100,
        'Rusty Chicken Parmesan',
        'Chicken Parmesan topped with a rusty tomato sauce and melted cheese, providing a rich and savory dinner.',
        'Dinner'
    ),
    (
        47,
        3200,
        'Rusty Lasagna',
        'Layered lasagna with a rusty sauce, providing a rich and flavorful dinner option.',
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
    (6, 4, 6, FALSE),
    -- Rusty Pancakes
    (7, 4, 7, TRUE),
    -- Rusty Omelette
    (11, 4, 11, FALSE),
    -- Rusty Bacon
    (12, 4, 12, TRUE),
    -- Rusty French Toast
    (13, 4, 13, FALSE),
    -- Rusty Waffles
    (14, 4, 14, FALSE),
    -- Rusty Cereal
    (15, 4, 15, FALSE),
    -- Rusty Smoothie
    (16, 4, 16, FALSE),
    -- Rusty Sausage
    (17, 4, 17, FALSE),
    -- Rusty Hash Browns
    (18, 4, 18, FALSE),
    -- Rusty Eggs Benedict
    (48, 4, 27, FALSE),
    -- Rusty Salmon
    (49, 4, 8, FALSE),
    -- Rusty Burger
    -- Rusty Lunch Menu
    (8, 5, 8, FALSE),
    -- Rusty Burger
    (9, 5, 9, FALSE),
    -- Rusty Salad
    (19, 5, 19, FALSE),
    -- Rusty Sandwich
    (20, 5, 20, FALSE),
    -- Rusty Soup
    (21, 5, 21, FALSE),
    -- Rusty Tacos
    (22, 5, 22, FALSE),
    -- Rusty Pizza
    (23, 5, 23, FALSE),
    -- Rusty Pasta
    (24, 5, 24, FALSE),
    -- Rusty Chicken Wrap
    (25, 5, 25, FALSE),
    -- Rusty Quesadilla
    (26, 5, 26, FALSE),
    -- Rusty Mac and Cheese
    (50, 5, 10, FALSE),
    -- Rusty Steak
    (51, 5, 12, FALSE),
    -- Rusty French Toast
    -- Rusty Dinner Menu
    (10, 6, 10, FALSE),
    -- Rusty Steak
    (27, 6, 27, FALSE),
    -- Rusty Salmon
    (28, 6, 28, FALSE),
    -- Rusty Lobster
    (29, 6, 29, FALSE),
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
    (35, 6, 35, FALSE),
    -- Rusty Meatloaf
    (55, 6, 43, FALSE),
    -- Rusty Prime Rib
    (56, 6, 44, FALSE),
    -- Rusty Pot Roast
    (57, 6, 45, FALSE),
    -- Rusty Stuffed Peppers
    (58, 6, 46, FALSE),
    -- Rusty Chicken Parmesan
    (59, 6, 47, FALSE);
-- Rusty Lasagna
-- Insert sample tags with explicit IDs
INSERT INTO tag (tag_id, name)
VALUES (5, 'Rusty'),
    (6, 'Iron-Rich'),
    (7, 'Unique'),
    (8, 'Chefs Special'),
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
    (36, 36, 5),
    (37, 37, 5),
    (38, 38, 5),
    (39, 39, 5),
    (40, 40, 5),
    (41, 41, 5),
    (42, 42, 5),
    (43, 43, 5),
    (44, 44, 5),
    (45, 45, 5),
    (46, 46, 5),
    (47, 47, 5),
    -- Iron-Rich dishes
    (48, 6, 6),
    (49, 7, 6),
    (50, 10, 6),
    (51, 11, 6),
    (52, 15, 6),
    (53, 16, 6),
    (54, 27, 6),
    (55, 28, 6),
    (56, 30, 6),
    (57, 32, 6),
    (58, 43, 6),
    (59, 44, 6),
    (60, 45, 6),
    (61, 46, 6),
    (62, 47, 6),
    -- Unique dishes
    (63, 6, 7),
    (64, 10, 7),
    (65, 13, 7),
    (66, 18, 7),
    (67, 21, 7),
    (68, 26, 7),
    (69, 29, 7),
    (70, 31, 7),
    (71, 34, 7),
    (72, 35, 7),
    (73, 36, 7),
    (74, 37, 7),
    (75, 38, 7),
    (76, 39, 7),
    (77, 40, 7),
    (78, 41, 7),
    (79, 42, 7),
    (80, 44, 7),
    (81, 47, 7),
    -- Chefs Special dishes
    (82, 6, 8),
    (83, 10, 8),
    (84, 13, 8),
    (85, 18, 8),
    (86, 21, 8),
    (87, 26, 8),
    (88, 29, 8),
    (89, 31, 8),
    (90, 34, 8),
    (91, 35, 8),
    (92, 36, 8),
    (93, 37, 8),
    (94, 39, 8),
    (95, 44, 8),
    (96, 47, 8),
    -- New dishes
    (97, 11, 9),
    (98, 12, 9),
    (99, 14, 9),
    (100, 15, 9),
    (101, 17, 9),
    (102, 19, 9),
    (103, 20, 9),
    (104, 22, 9),
    (105, 23, 9),
    (106, 25, 9),
    (107, 38, 9),
    (108, 40, 9),
    (109, 41, 9),
    (110, 42, 9),
    (111, 45, 9),
    (112, 46, 9),
    (113, 47, 9);
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
-- Update sequence for menucard table
ALTER SEQUENCE menucard_menucard_id_seq RESTART WITH 7;
-- Update sequence for setting table (assuming there's a sequence, adjust if necessary)
ALTER SEQUENCE setting_setting_id_seq RESTART WITH 3;
-- Update sequence for dish table
ALTER SEQUENCE dish_dish_id_seq RESTART WITH 48;
-- Update sequence for menucard_dish table
ALTER SEQUENCE menucard_dish_menucard_dish_id_seq RESTART WITH 60;
-- Update sequence for tag table
ALTER SEQUENCE tag_tag_id_seq RESTART WITH 10;
-- Update sequence for dish_tag table
ALTER SEQUENCE dish_tag_dish_tag_id_seq RESTART WITH 114;
-- Update sequence for table table (assuming there's a sequence, adjust if necessary)
ALTER SEQUENCE table_table_id_seq RESTART WITH 7;
-- Update sequence for person table
ALTER SEQUENCE person_person_id_seq RESTART WITH 7;
-- Update sequence for reservation table
ALTER SEQUENCE reservation_reservation_id_seq RESTART WITH 7;
-- Update sequence for table_reservation table
ALTER SEQUENCE table_reservation_table_reservation_id_seq RESTART WITH 7;
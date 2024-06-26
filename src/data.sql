-- Insert sample menucards with explicit IDs
INSERT INTO menucard (menucard_id, name)
VALUES (1, 'Breakfast Menu'),
    (2, 'Lunch Menu'),
    (3, 'Dinner Menu');
ALTER SEQUENCE menucard_menucard_id_seq RESTART WITH 4;
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
ALTER SEQUENCE dish_dish_id_seq RESTART WITH 6;
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
ALTER SEQUENCE menucard_dish_menucard_dish_id_seq RESTART WITH 6;
-- Steak on Dinner Menu
-- Insert sample tags with explicit IDs
INSERT INTO tag (tag_id, name)
VALUES (1, 'Gluten-Free'),
    (2, 'Vegan'),
    (3, 'Spicy'),
    (4, 'Popular');
ALTER SEQUENCE tag_tag_id_seq RESTART WITH 5;
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
ALTER SEQUENCE dish_tag_dish_tag_id_seq RESTART WITH 6;
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
ALTER SEQUENCE table_table_id_seq RESTART WITH 4;
-- Insert sample persons with explicit IDs
INSERT INTO person (person_id, name, phone)
VALUES (1, 'John Doe', '555-1234'),
    (2, 'Jane Smith', '555-5678'),
    (3, 'Alice Johnson', '555-9101');
ALTER SEQUENCE person_person_id_seq RESTART WITH 4;
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
ALTER SEQUENCE reservation_reservation_id_seq RESTART WITH 4;
-- Insert sample table reservations with explicit IDs
INSERT INTO table_reservation (table_reservation_id, id_table, id_reservation)
VALUES (1, 1, 1),
    (2, 2, 2),
    (3, 3, 3);
ALTER SEQUENCE table_reservation_table_reservation_id_seq RESTART WITH 4;
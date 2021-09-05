CREATE TABLE battle (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    war_num INTEGER NOT NULL,
    map_location VARCHAR(64) NOT NULL,
    name VARCHAR(32),
    description VARCHAR(2000),
    last_edited DATETIME,
    submitted DATETIME NOT NULL,
    FOREIGN KEY (war_num) REFERENCES war(num)
);

-- the id is nicknamed the "battle id"
-- war_num is the war number this took place in, foreign key to war
-- map_location is a game location is defined by api
-- name and description is user-made
-- last_edited was the last time user-made values where edited
-- submitted was when it was submitted to database
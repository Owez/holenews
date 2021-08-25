CREATE TABLE battle (
    id INTEGER PRIMARY KEY NOT NULL,
    war_num INTEGER NOT NULL,
    location_hex VARCHAR(64) NOT NULL,
    location_inner VARCHAR(64) NOT NULL,
    name VARCHAR(32),
    description VARCHAR(2000),
    last_edited TIMESTAMP WITH TIME ZONE,
    submitted TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (war_num) REFERENCES war(num)
);

-- the id is nicknamed the "battle id"
-- war_num is the war number this took place in, foreign key to war
-- location_hex is a game hex is defined by api
-- location_inner is the location inside of location_hex defined by api
-- name and description is user-made
-- last_edited was the last time user-made values where edited
-- submitted was when it was submitted to database
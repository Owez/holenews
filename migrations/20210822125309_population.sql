CREATE TABLE population (
    battle_id INTEGER NOT NULL,
    counted INTEGER NOT NULL,
    at_time DATETIME NOT NULL,
    description VARCHAR(150),
    last_edited DATETIME,
    submitted DATETIME NOT NULL,
    PRIMARY KEY (battle_id, at_time),
    FOREIGN KEY (battle_id) REFERENCES battle(id)
);

-- battle id is foreign id of a battle
-- counted is population count
-- at_time is the time the population was counted
-- description is user-made
-- last_edited was the last time user-made values where edited
-- submitted was when it was submitted to database
-- the primary key is a composite of the battle id and the time of the count
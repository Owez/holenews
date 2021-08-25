CREATE TABLE war (
    num INTEGER PRIMARY KEY NOT NULL,
    time_start TIMESTAMP WITH TIME ZONE NOT NULL,
    time_end TIMESTAMP WITH TIME ZONE,
    colonial_win BOOLEAN,
    submitted TIMESTAMP WITH TIME ZONE NOT NULL
);

-- num is the war number and primary key
-- time_start is the known start time
-- time_end is the end time if it's ended
-- colonial_win indicates if the colonials won the war
-- submitted was when it was submitted to database
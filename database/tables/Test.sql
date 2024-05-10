-- Create a postgres table named test that has an auto-incrementing id column and another varchar column called name and description
CREATE TABLE IF NOT EXISTS Test (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    description TEXT
);
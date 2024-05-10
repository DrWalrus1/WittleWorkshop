-- Create a postgres function if it doesnt exist that inserts a row into the test table
CREATE OR REPLACE FUNCTION 
InsertIntoTest(name VARCHAR(255), description TEXT) 
RETURNS VOID AS
$$
BEGIN
    INSERT INTO Test (name, description) VALUES (name, description);
END;
$$
LANGUAGE plpgsql;
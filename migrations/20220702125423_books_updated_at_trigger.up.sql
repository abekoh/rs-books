-- Add up migration script here
CREATE TRIGGER books_set_timestamp
    BEFORE UPDATE
    ON books
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
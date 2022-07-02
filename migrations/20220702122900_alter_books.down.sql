-- Add down migration script here
ALTER TABLE books
    DROP COLUMN created_at,
    DROP COLUMN updated_at,
    DROP COLUMN published_year,
    DROP COLUMN original_published_year
;

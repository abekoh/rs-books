-- Add up migration script here
ALTER TABLE books
    ADD COLUMN created_at              timestamp with time zone NOT NULL DEFAULT now(),
    ADD COLUMN updated_at              timestamp with time zone NOT NULL DEFAULT now(),
    ADD COLUMN published_year          integer,
    ADD COLUMN original_published_year integer
;

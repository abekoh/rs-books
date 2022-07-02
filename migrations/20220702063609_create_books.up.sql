-- Add up migration script here
CREATE TABLE books (id UUID PRIMARY KEY, name VARCHAR NOT NULL, url VARCHAR);
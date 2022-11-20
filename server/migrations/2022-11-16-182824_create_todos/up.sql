-- Your SQL goes here

CREATE TABLE
    todos (
        id SERIAL PRIMARY KEY,
        title CHAR(250) NOT NULL,
        description CHAR(500) NOT NULL,
        is_completed BOOLEAN NOT NULL DEFAULT FALSE
    );
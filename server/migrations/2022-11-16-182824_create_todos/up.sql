-- Your SQL goes here

CREATE TABLE
    todos (
        id INT PRIMARY KEY NOT NULL,
        title CHAR(250) NOT NULL,
        description CHAR(500) NOT NULL,
        is_completed BOOLEAN NOT NULL DEFAULT FALSE
    );
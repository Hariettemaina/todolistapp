-- Your SQL goes here
CREATE TABLE todo (
    id SERIAL PRIMARY KEY NOT NULL,
    title VARCHAR(100) NOT NULL,
    contents VARCHAR(512) NOT NULL,
    completed BOOLEAN NOT NULL,
    date_created Date NOT NULL
    )
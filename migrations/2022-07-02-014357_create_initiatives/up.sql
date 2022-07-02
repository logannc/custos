-- Your SQL goes here
CREATE TABLE initiatives (
    id INTEGER NOT NULL PRIMARY KEY,
    player VARCHAR NOT NULL,
    modifier INTEGER NOT NULL,
    current_init INTEGER NOT NULL
)
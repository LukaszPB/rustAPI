DROP TABLE IF EXISTS book;
CREATE TABLE Author (
    id BIGSERIAL PRIMARY KEY,
    first_name VARCHAR,
    last_name VARCHAR,
    age INT
);
CREATE TABLE Book (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR,
    id_author BIGINT,
    FOREIGN KEY (id_author) REFERENCES Author(id)
);
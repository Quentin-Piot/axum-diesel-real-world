CREATE TABLE posts
(
    id        uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)

CREATE TABLE posts
(
    id        bigint auto_increment PRIMARY KEY,
    title     VARCHAR(256) NOT NULL,
    body      text    NOT NULL,
    published bool NOT NULL DEFAULT FALSE
)

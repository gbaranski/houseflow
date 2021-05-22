CREATE TABLE IF NOT EXISTS users (
    id            USER_ID NOT NULL,
    first_name    TEXT    NOT NULL,
    last_name     TEXT    NOT NULL,
    email         TEXT    NOT NULL,
    password_hash TEXT    NOT NULL,

    PRIMARY KEY(
      id
    )
);

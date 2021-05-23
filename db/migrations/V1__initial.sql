CREATE TABLE users (
    id            VARCHAR(32) NOT NULL,
    first_name    TEXT        NOT NULL,
    last_name     TEXT        NOT NULL,
    email         TEXT        NOT NULL,
    password_hash TEXT        NOT NULL,

    PRIMARY KEY(
      id
    )
);

CREATE EXTENSION hstore;

CREATE TABLE devices (
    id              VARCHAR(32) NOT NULL,
    password_hash   TEXT        NOT NULL,
    type            TEXT        NOT NULL,
    traits          TEXT[]      NOT NULL,
    name            TEXT        NOT NULL,
    will_push_state BOOL        NOT NULL,
    room_hint       TEXT,
    model           TEXT        NOT NULL,
    hw_version      TEXT        NOT NULL,
    sw_version      TEXT        NOT NULL,
    attributes      hstore      NOT NULL,

    PRIMARY KEY (
      id
    )
);


CREATE TABLE user_devices (
    user_id   VARCHAR(32)   REFERENCES users (id) ON DELETE CASCADE,
    device_id VARCHAR(32)   REFERENCES devices (id) ON DELETE CASCADE,
    read      BOOL          NOT NULL,
    write     BOOL          NOT NULL,
    execute   BOOL          NOT NULL,

    PRIMARY KEY(user_id, device_id)
);

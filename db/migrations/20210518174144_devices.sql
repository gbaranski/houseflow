CREATE EXTENSION IF NOT EXISTS hstore;

CREATE TABLE IF NOT EXISTS devices (
    id              DEVICE_ID         NOT NULL,
    password_hash   DEVICE_PASSWORD   NOT NULL,
    type            TEXT              NOT NULL,
    traits          TEXT[]            NOT NULL,
    name            TEXT              NOT NULL,
    will_push_state BOOL              NOT NULL,
    room_hint       TEXT,
    model           TEXT              NOT NULL,
    hw_version      TEXT              NOT NULL,
    sw_version      TEXT              NOT NULL,
    attributes      hstore            NOT NULL,

    PRIMARY KEY (
      id
    )
);

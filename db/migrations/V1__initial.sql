CREATE EXTENSION hstore;

CREATE TABLE users (
  id            CHAR(32)        NOT NULL,
  username      TEXT            NOT NULL,
  email         TEXT     UNIQUE NOT NULL,
  password_hash TEXT            NOT NULL,

  PRIMARY KEY (
    id
  )
);

CREATE TABLE user_structures (
  structure_id  CHAR(32) REFERENCES structures (id) ON DELETE CASCADE,
  user_id       CHAR(32) REFERENCES users      (id) ON DELETE CASCADE,
  manager       BOOL     NOT NULL,

  PRIMARY KEY (
    structure_id,
    user_id,
  )
);


CREATE TABLE structures (
  id          CHAR(32)  UNIQUE NOT NULL,
  label       TEXT      NOT NULL,

  PRIMARY KEY (
    id
  )
);


CREATE TABLE rooms (
  room_id      CHAR(32) UNIQUE NOT NULL,
  structure_id CHAR(32) REFERENCES structures (id) ON DELETE CASCADE,
  label        TEXT     NOT NULL,

  PRIMARY KEY (
    room_id
  )
);

CREATE TABLE devices (
  id              CHAR(32)    UNIQUE NOT NULL,
  room_id         CHAR(32)    REFERENCES room (id) ON DELETE CASCADE,
  password_hash   TEXT        NOT NULL,
  type            TEXT        NOT NULL,
  traits          TEXT[]      NOT NULL,
  name            TEXT        NOT NULL,
  will_push_state BOOL        NOT NULL,
  model           TEXT        NOT NULL,
  hw_version      TEXT        NOT NULL,
  sw_version      TEXT        NOT NULL,
  attributes      hstore      NOT NULL,

  PRIMARY KEY (
    id,
    structure_id
  )
);
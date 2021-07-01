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

CREATE TABLE admins (
  user_id CHAR(32) REFERENCES users (id) ON DELETE CASCADE,

  PRIMARY KEY (
    user_id
  )
);

CREATE TABLE structures (
  id          CHAR(32)  UNIQUE NOT NULL,
  name        TEXT             NOT NULL,

  PRIMARY KEY (
    id
  )
);

CREATE TABLE user_structures (
  structure_id  CHAR(32) REFERENCES structures (id) ON DELETE CASCADE,
  user_id       CHAR(32) REFERENCES users      (id) ON DELETE CASCADE,
  is_manager    BOOL     NOT NULL,

  PRIMARY KEY (
    structure_id,
    user_id
  )
);


CREATE TABLE rooms (
  id           CHAR(32) UNIQUE NOT NULL,
  structure_id CHAR(32) REFERENCES structures (id) ON DELETE CASCADE,
  name         TEXT     NOT NULL,

  PRIMARY KEY (
    id
  )
);

CREATE TABLE devices (
  id              CHAR(32)    UNIQUE NOT NULL,
  room_id         CHAR(32)    REFERENCES rooms (id) ON DELETE CASCADE,
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
    id
  )
);

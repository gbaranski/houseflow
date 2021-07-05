-- insert into users values('faf04931e9f14f0da3a59d3e51375e6e', 'gbaranski', 'root@gbaranski.com', 'phash');
-- insert into admins values('faf04931e9f14f0da3a59d3e51375e6e');

CREATE TABLE users (
  id            CHAR(32) NOT NULL,
  username      VARCHAR  NOT NULL,
  email         VARCHAR  NOT NULL UNIQUE,
  password_hash VARCHAR  NOT NULL,

  CHECK( length(id) == 32 )

  PRIMARY KEY( id )
);

CREATE TABLE admins (
  user_id CHAR(32) NOT NULL REFERENCES users( id ),

  PRIMARY KEY( user_id )
);

CREATE TABLE structures (
  id          CHAR(32)   NOT NULL UNIQUE,
  name        VARCHAR    NOT NULL,

  CHECK( length(id) == 32 )

  PRIMARY KEY( id )
);

CREATE TABLE user_structures (
  structure_id  CHAR(32) NOT NULL REFERENCES structures(id) ON DELETE CASCADE,
  user_id       CHAR(32) NOT NULL REFERENCES users     (id) ON DELETE CASCADE,
  is_manager    BOOLEAN  NOT NULL,

  CHECK( is_manager in (0, 1) )

  PRIMARY KEY( structure_id, user_id )
);


CREATE TABLE rooms (
  id           CHAR(32) NOT NULL,
  structure_id CHAR(32) NOT NULL REFERENCES structures(id) ON DELETE CASCADE,
  name         VARCHAR  NOT NULL,

  PRIMARY KEY( id )
);

CREATE TABLE devices (
  id              CHAR(32)    NOT NULL,
  room_id         CHAR(32)    NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
  password_hash   VARCHAR     NOT NULL, -- password hash used to verify
  type            VARCHAR     NOT NULL, -- type of the device
  name            VARCHAR     NOT NULL, -- name of the device
  will_push_state BOOLEAN     NOT NULL, -- true if device will push state on it's own
  model           VARCHAR     NOT NULL, -- model name of the device
  hw_version      VARCHAR     NOT NULL, -- hardware version, must follow semver
  sw_version      VARCHAR     NOT NULL, -- software version, must follow semver
  attributes      VARCHAR     NOT NULL, -- device attributes in JSON format

  CHECK( will_push_state in (0, 1) )

  PRIMARY KEY( id )
);

CREATE TABLE device_traits (
  device_id  CHAR(32) NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
  trait_name VARCHAR  NOT NULL,
  
  PRIMARY KEY(device_id, trait_name)
);


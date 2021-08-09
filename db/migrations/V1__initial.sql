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

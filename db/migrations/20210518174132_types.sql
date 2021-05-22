CREATE DOMAIN DEVICE_ID       as VARCHAR(32); -- 16 bytes = 32 in hex
CREATE DOMAIN DEVICE_PASSWORD as TEXT;        -- Argon2 hashed password

CREATE DOMAIN USER_ID         as VARCHAR(32); -- 16 bytes = 32 in hex
CREATE DOMAIN USER_PASSWORD   as TEXT;        -- Argon2 hashed password

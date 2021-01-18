
CREATE TABLE IF NOT EXISTS devices (
    id 					UUID      PRIMARY KEY,

    -- Base64 encoded ed25519 public key
    -- 4 * ceil(ED25519_PKEY_BYTES / 3) = 44
	publickey 			CHAR(44)  NOT NULL,

    -- Type of the device, must be one of those https://developers.google.com/assistant/smarthome/guides
	type				TEXT 	  NOT NULL,

    -- Tells if device will report his state on his own, or should use polling
    will_report_state	BOOL 	  NOT NULL,

    -- e.g Garage
    room_hint			TEXT      NOT NULL,

    -- e.g gbaranski's garage
    manufacturer        TEXT      NOT NULL,
    -- e.g nightlamp
    model               TEXT      NOT NULL,

    -- e.g 2.0.5
    hw_version          TEXT      NOT NULL,
    sw_version          TEXT      NOT NULL,
)

CREATE TABLE IF NOT EXISTS device_traits (
    id          UUID PRIMARY KEY,

    -- Name of the trait, must be one of those https://developers.google.com/assistant/smarthome/traits
    name        TEXT NOT NULL,

    device_id   UUID REFERENCES devices,
)


CREATE TABLE IF NOT EXISTS users (
    id          UUID    PRIMARY KEY,

    first_name  TEXT    NOT NULL,
    last_name   TEXT    NOT NULL,
    email       TEXT    NOT NULL,
    password    TEXT    NOT NULL,
)

CREATE TABLE IF NOT EXISTS user_devices (
    id          UUID    PRIMARY KEY,

    user_id     UUID    REFERENCES users   (id),
    device_id   UUID    REFERENCES devices (id),

    read        BOOL    NOT NULL,
    write       BOOL    NOT NULL,
    execute     BOOL    NOT NULL,
)
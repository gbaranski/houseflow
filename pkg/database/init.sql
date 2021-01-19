
CREATE TABLE IF NOT EXISTS devices (
    id 					UUID      PRIMARY KEY,
	publickey           CHAR(44)  NOT NULL, -- Base64 encoded ed25519 public key, size = 4 * ceil(PKEY_BYTES / 3) = 44
	type				TEXT 	  NOT NULL, -- Type of the device, must be one of those https://developers.google.com/assistant/smarthome/guides
    will_report_state	BOOL 	  NOT NULL, -- True if device will report it state, false if use polling
    room_hint			TEXT      NOT NULL, -- e.g Bedroom
    manufacturer        TEXT      NOT NULL, -- e.g gbaranski's garage
    model               TEXT      NOT NULL, -- e.g nightlamp
    hw_version          TEXT      NOT NULL, -- e.g 2.0.5
    sw_version          TEXT      NOT NULL, -- e.g 3.2.0
)

CREATE TABLE IF NOT EXISTS device_traits (
    id          UUID PRIMARY KEY,
    device_id   UUID REFERENCES devices (id),
    name        TEXT NOT NULL, -- Name of the trait, must be one of those https://developers.google.com/assistant/smarthome/traits
)


CREATE TABLE IF NOT EXISTS users (
    id              UUID    PRIMARY KEY,
    first_name      TEXT    NOT NULL,
    last_name       TEXT    NOT NULL,
    email           TEXT    NOT NULL,
    password_hash   TEXT    NOT NULL, -- bcrypted password
)

CREATE TABLE IF NOT EXISTS user_devices (
    user_id     UUID    REFERENCES users   (id),
    device_id   UUID    REFERENCES devices (id),
    read        BOOL    NOT NULL, -- determines if user can do query intent
    write       BOOL    NOT NULL, -- unused at the moment, probably cna be used to tell if user can invite someone
    execute     BOOL    NOT NULL, -- determines if user can do execute intent

    PRIMARY KEY (user_id, device_id)
)
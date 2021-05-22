CREATE TABLE IF NOT EXISTS user_permissions (
    user_id   USER_ID   REFERENCES users (id) ON DELETE CASCADE,
    device_id DEVICE_ID REFERENCES devices (id) ON DELETE CASCADE,
    read      BOOL      NOT NULL,
    write     BOOL      NOT NULL,
    execute   BOOL      NOT NULL,

    PRIMARY KEY(user_id, device_id)
);

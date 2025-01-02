
-- USERS

CREATE TABLE users (
    id text NOT NULL,
    password_hash text,
    creation_ts bigint,
    admin boolean DEFAULT false NOT NULL,
    upgrade_ts bigint,
    is_guest boolean DEFAULT false NOT NULL,
    appservice_id text,
    deactivated boolean DEFAULT false NOT NULL,
    PRIMARY KEY (id)
);

-- ACCESS TOKENS

CREATE TABLE access_tokens (
    id text NOT NULL,
    user_id text NOT NULL,
    device_id text,
    valid_until_ms bigint,
    puppets_user_id text,
    last_validated bigint,
    refresh_token_id bigint,
    used boolean,
    PRIMARY KEY (id)
);

-- DEVICES

CREATE TABLE devices (
    id text NOT NULL,
    user_id text NOT NULL,
    display_name text,
    last_seen bigint,
    hidden boolean DEFAULT false,
    PRIMARY KEY (id)
);

-- PROFILES

CREATE TABLE profiles (
    id text NOT NULL,
    displayname text,
    avatar_url text,
    PRIMARY KEY (id)
);

-- ACCOUNT DATA

CREATE TABLE account_data (
    id text NOT NULL,
    kind text NOT NULL,
    content text NOT NULL,
    instance_name text,
    PRIMARY KEY (id)
);

-- REGISTRATION TOKENS

CREATE TABLE registration_tokens (
    id text NOT NULL,
    uses_allowed integer,
    pending integer NOT NULL,
    completed integer NOT NULL,
    expiry_time bigint,
    PRIMARY KEY (id)
);

-- REFRESH TOKENS

CREATE TABLE refresh_tokens (
    id text NOT NULL,
    user_id text NOT NULL,
    device_id text NOT NULL,
    expiry_ts bigint,
    PRIMARY KEY (id)
);

-- UIAA SESSIONS

CREATE TABLE uiaa_sessions (
    id text NOT NULL,
    creation_time bigint NOT NULL,
    json text NOT NULL,
    -- uri text NOT NULL,
    -- method text NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE uiaa_credentials (
    id text NOT NULL,
    stage_type text NOT NULL,
    result text NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (id)
        REFERENCES uiaa_sessions(id) ON DELETE CASCADE
);

CREATE TABLE server_keys_json (
    id text NOT NULL,
    server_name text NOT NULL,
    from_server text NOT NULL,
    ts_added_ms bigint NOT NULL,
    ts_valid_until_ms bigint NOT NULL,
    key_json bytea NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE server_signature_keys (
    id text NOT NULL,
    server_name text,
    from_server text,
    ts_added_ms bigint,
    verify_key bytea,
    ts_valid_until_ms bigint,
    PRIMARY KEY (id)
);


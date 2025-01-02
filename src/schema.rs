// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Text,
        user_id -> Text,
        device_id -> Nullable<Text>,
        valid_until_ms -> Nullable<Int8>,
        puppets_user_id -> Nullable<Text>,
        last_validated -> Nullable<Int8>,
        refresh_token_id -> Nullable<Int8>,
        used -> Nullable<Bool>,
    }
}

diesel::table! {
    account_data (id) {
        id -> Text,
        kind -> Text,
        content -> Text,
        instance_name -> Nullable<Text>,
    }
}

diesel::table! {
    devices (id) {
        id -> Text,
        user_id -> Text,
        display_name -> Nullable<Text>,
        last_seen -> Nullable<Int8>,
        hidden -> Nullable<Bool>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Text,
        displayname -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Text,
        user_id -> Text,
        device_id -> Text,
        expiry_ts -> Nullable<Int8>,
    }
}

diesel::table! {
    registration_tokens (id) {
        id -> Text,
        uses_allowed -> Nullable<Int4>,
        pending -> Int4,
        completed -> Int4,
        expiry_time -> Nullable<Int8>,
    }
}

diesel::table! {
    server_keys_json (id) {
        id -> Text,
        server_name -> Text,
        from_server -> Text,
        ts_added_ms -> Int8,
        ts_valid_until_ms -> Int8,
        key_json -> Bytea,
    }
}

diesel::table! {
    server_signature_keys (id) {
        id -> Text,
        server_name -> Nullable<Text>,
        from_server -> Nullable<Text>,
        ts_added_ms -> Nullable<Int8>,
        verify_key -> Nullable<Bytea>,
        ts_valid_until_ms -> Nullable<Int8>,
    }
}

diesel::table! {
    uiaa_credentials (id) {
        id -> Text,
        stage_type -> Text,
        result -> Text,
    }
}

diesel::table! {
    uiaa_sessions (id) {
        id -> Text,
        creation_time -> Int8,
        json -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        password_hash -> Nullable<Text>,
        creation_ts -> Nullable<Int8>,
        admin -> Bool,
        upgrade_ts -> Nullable<Int8>,
        is_guest -> Bool,
        appservice_id -> Nullable<Text>,
        deactivated -> Bool,
    }
}

diesel::joinable!(uiaa_credentials -> uiaa_sessions (id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    account_data,
    devices,
    profiles,
    refresh_tokens,
    registration_tokens,
    server_keys_json,
    server_signature_keys,
    uiaa_credentials,
    uiaa_sessions,
    users,
);

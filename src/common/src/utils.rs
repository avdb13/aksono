use argon2::{
    password_hash::{PasswordHashString, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::{distributions::Alphanumeric, Rng};

pub fn secure_rand_str(length: usize) -> String {
    let it = rand::thread_rng().sample_iter(&Alphanumeric);

    it.take(length).map(char::from).collect()
}

pub fn utc_timestamp_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// TODO: configuration and unicode normalization
pub fn hash_password<B>(password: B) -> Result<PasswordHashString, argon2::password_hash::Error>
where
    B: AsRef<[u8]>,
{
    Argon2::default()
        .hash_password(
            password.as_ref(),
            &SaltString::generate(&mut rand::thread_rng()),
        )
        .map(|x| x.serialize())
}

pub fn verify_password<B, S>(encoded: S, password: B) -> bool
where
    B: AsRef<[u8]>,
    S: AsRef<str>,
{
    let Ok(password_hash) = PasswordHash::new(encoded.as_ref()) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_ref(), &password_hash)
        .is_ok()
}

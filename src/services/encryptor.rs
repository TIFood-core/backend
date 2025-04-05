use std::sync::OnceLock;

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

static ENCRYPTOR: OnceLock<Argon2> = OnceLock::new();

fn get_encryptor() -> &'static Argon2<'static> {
    ENCRYPTOR.get_or_init(|| Argon2::default())
}

pub fn encrypt_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    get_encryptor()
        .hash_password(&password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

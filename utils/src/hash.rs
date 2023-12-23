use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,

};


pub fn argon_hash(content: impl AsRef<str>) -> Result<String , argon2::password_hash::Error>{

    let salt =SaltString::generate(&mut OsRng);
    let aragon=Argon2::default();
    Ok(
        aragon
            .hash_password(content.as_ref().as_bytes(), &salt)?
            .to_string(),
    )

}


#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use super::*;

    #[test]
    pub fn test_argon_hash() {
        let password: String = Faker.fake();
        let hash_pass = argon_hash(password).unwrap();
        assert!(!hash_pass.is_empty());
    }
}
use test_context::futures::future::ok;
use super::hash;
use errors::{invalid_input_error, AppResult};
use tracing::debug;




pub async fn hash_password(password: String) -> AppResult<String> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    let password = jh.await??;
    Ok(password)
}
#[cfg(test)]

mod  tests{
    use fake::{
        Fake , Faker
    };
    use super::*;
    #[tokio::test]


    pub async fn hash_password_test() {
        let password: String = Faker.fake();
        let hash_pass = hash_password(password).await.unwrap();
        assert!(!hash_pass.is_empty());
    }
}
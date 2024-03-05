use rand::RngCore;

use argon2::{self, Config};

use crate::error::{AppError, AppErrorType};

pub async fn hash_password(plain_password: &String, config: &Config<'_>) -> Result<String , AppError> {
    let salt = generate_salt().await;
    argon2::hash_encoded(plain_password.as_bytes(), &salt, config)
        .map_err(|_| AppError::new(None, Some(String::from("Could not encode password!")), AppErrorType::InternalServerError))
}

pub async fn verify_password(hash: &String, password: &String) -> Result<bool , AppError> {
    argon2::verify_encoded(hash, password.as_bytes())
    .map_err(|_| AppError::new(None, Some(String::from("Could not decode password!")), AppErrorType::InternalServerError))
}

async fn generate_salt() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut salt = [0u8; 16];
    rng.fill_bytes(&mut salt);
    salt
}

#[cfg(test)]
mod util_tests {
    use super::*;
    use argon2::{self, Config};

    #[actix_rt::test]
    pub async fn hash_password_returns_ok() {
        // given
        let password: String = "Pass12345".to_string();
        let config = Config::default();

        // when
        let hash = hash_password(&password, &config).await.unwrap();
    
        dbg!("{:?}", &hash);
        // then
    
        let matches = verify_password(&hash, &password).await.unwrap();
        assert!(matches);
    }

    #[actix_rt::test]
    async fn test_generate_salt() {
        // Generate two salts
        let salt1 = generate_salt().await;
        let salt2 = generate_salt().await;

        // Ensure salts are not all zeros
        assert_ne!(salt1, [0u8; 16]);
        assert_ne!(salt2, [0u8; 16]);

        // Ensure salts are not equal
        assert_ne!(salt1, salt2);
    }
}
use std::{collections::HashSet, env};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

pub struct JwtAuthentication {
    secret: String,
}

impl JwtAuthentication {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self { secret }
    }

    pub fn create_token<T>(&self, obj: &T) -> Result<String, jsonwebtoken::errors::Error>
    where
        T: Serialize,
    {
        let token = encode(
            &Header::default(),
            &obj,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );
        println!("Token {:?}", token);
        token
    }

    pub fn validate<T: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<T, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = false;
        validation.required_spec_claims = HashSet::new();
        let decoded = decode::<T>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        );
        decoded.map(|data| data.claims)
    }
}

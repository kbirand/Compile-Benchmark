use crate::error::{AppError, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

pub struct AuthService {
    jwt_secret: String,
    access_token_expiry: i64,
    refresh_token_expiry: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, access_token_expiry: i64, refresh_token_expiry: i64) -> Self {
        Self {
            jwt_secret,
            access_token_expiry,
            refresh_token_expiry,
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalError(format!("Invalid password hash: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
    ) -> Result<TokenPair> {
        let now = Utc::now();
        let access_exp = now + Duration::seconds(self.access_token_expiry);
        let refresh_exp = now + Duration::seconds(self.refresh_token_expiry);

        let access_claims = Claims {
            sub: user_id.to_string(),
            user_id,
            email: email.to_string(),
            role: role.to_string(),
            exp: access_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        let refresh_claims = RefreshClaims {
            sub: user_id.to_string(),
            user_id,
            exp: refresh_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_expiry,
        })
    }

    pub fn validate_access_token(&self, token: &str) -> Result<TokenData<Claims>> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<TokenData<RefreshClaims>> {
        decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid refresh token: {}", e)))
    }

    pub fn extract_token_from_header(auth_header: &str) -> Result<&str> {
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::AuthenticationError(
                "Invalid authorization header format".to_string(),
            ));
        }
        Ok(&auth_header[7..])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: AuthUserInfo,
    pub tokens: TokenPair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUserInfo {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub role: String,
}

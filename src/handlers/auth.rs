use axum::{extract::State, Json};

use crate::{
    auth::{
        AuthResponse, AuthService, AuthUserInfo, LoginRequest, RefreshTokenRequest,
        RegisterRequest, TokenPair,
    },
    error::{AppError, Result},
    services::user_service::UserService,
    AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let user_service = UserService::new(state.db.clone(), state.cache.clone());
    let auth_service = AuthService::new(
        state.config.auth.jwt_secret.clone(),
        state.config.auth.token_expiry_seconds,
        state.config.auth.refresh_token_expiry_seconds,
    );

    let user = user_service
        .get_user_by_email(&request.email)
        .await?
        .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    // Verify password
    if !auth_service.verify_password(&request.password, &user.password_hash)? {
        return Err(AppError::AuthenticationError(
            "Invalid credentials".to_string(),
        ));
    }

    // Generate tokens
    let tokens = auth_service.generate_token_pair(
        user.id,
        &user.email,
        &format!("{:?}", user.role).to_lowercase(),
    )?;

    Ok(Json(AuthResponse {
        user: AuthUserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            role: format!("{:?}", user.role).to_lowercase(),
        },
        tokens,
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    let user_service = UserService::new(state.db.clone(), state.cache.clone());
    let auth_service = AuthService::new(
        state.config.auth.jwt_secret.clone(),
        state.config.auth.token_expiry_seconds,
        state.config.auth.refresh_token_expiry_seconds,
    );

    // Check if email already exists
    if user_service.get_user_by_email(&request.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Check if username already exists
    if user_service
        .get_user_by_username(&request.username)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = auth_service.hash_password(&request.password)?;

    // Create user
    let user = user_service
        .create_user_with_password(
            request.email.clone(),
            request.username.clone(),
            password_hash,
            request.first_name,
            request.last_name,
        )
        .await?;

    // Generate tokens
    let tokens = auth_service.generate_token_pair(
        user.id,
        &user.email,
        &format!("{:?}", user.role).to_lowercase(),
    )?;

    Ok(Json(AuthResponse {
        user: AuthUserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            role: format!("{:?}", user.role).to_lowercase(),
        },
        tokens,
    }))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<TokenPair>> {
    let auth_service = AuthService::new(
        state.config.auth.jwt_secret.clone(),
        state.config.auth.token_expiry_seconds,
        state.config.auth.refresh_token_expiry_seconds,
    );
    let user_service = UserService::new(state.db.clone(), state.cache.clone());

    // Validate refresh token
    let token_data = auth_service.validate_refresh_token(&request.refresh_token)?;

    // Get user
    let user = user_service
        .get_user_by_id(token_data.claims.user_id)
        .await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;

    // Generate new token pair
    let tokens = auth_service.generate_token_pair(
        user.id,
        &user.email,
        &format!("{:?}", user.role).to_lowercase(),
    )?;

    Ok(Json(tokens))
}

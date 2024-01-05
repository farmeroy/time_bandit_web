use axum::{debug_handler, extract::State, Json};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    PrivateCookieJar,
};
use http::StatusCode;
use tracing::info;

use crate::{internal_error, models::LoginDetails, AppState};

pub async fn register_user(
    State(state): State<AppState>,
    Json(new_user): Json<LoginDetails>,
) -> Result<Json<String>, (StatusCode, String)> {
    let res = state
        .store
        .register_account(new_user)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

#[debug_handler(state = AppState)]
pub async fn login(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    Json(login): Json<LoginDetails>,
) -> Result<(PrivateCookieJar, StatusCode), StatusCode> {
    let user = match state.store.clone().get_account(login.email).await {
        Ok(user) => Ok(user),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    };
    match user {
        Ok(user) => match state.store.create_session(user, login.password).await {
            Ok(session_id) => {
                let cookie = Cookie::build(("time_bandit_auth_token_v1", session_id.0))
                    .secure(true)
                    .same_site(SameSite::Strict)
                    .http_only(true)
                    .path("/")
                    .build();
                Ok((jar.add(cookie), StatusCode::OK))
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        },
        Err(e) => Err(e),
    }
}

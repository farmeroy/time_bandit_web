use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::PrivateCookieJar;
use http::StatusCode;
use sqlx::postgres::PgRow;
use sqlx::Row;
use tracing::info;

use crate::{internal_error, models::UserId, AppState};

/// A simple endpoint to check if the cookie session is valid
/// This is used in a <Session/> wrapper in the frontend
pub async fn get_session(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<Json<UserId>, (StatusCode, String)> {
    let Some(cookie) = jar
        .get("time_bandit_auth_token_v1")
        .map(|cookie| cookie.value().to_owned())
    else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    };
    let res = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
        .bind(cookie)
        .map(|row: PgRow| UserId(row.get("user_id")))
        .fetch_one(&state.store.connection)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    // we create a mutable request in order to add extensions
    mut request: Request,
    // this allows us to call the request
    next: Next,
) -> Result<(PrivateCookieJar, Response), Response> {
    let Some(cookie) = jar
        .get("time_bandit_auth_token_v1")
        .map(|cookie| cookie.value().to_owned())
    else {
        let res = (StatusCode::UNAUTHORIZED).into_response();
        info!("{:?}", res);
        return Err(res);
    };
    let find_session = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
        .bind(cookie)
        .map(|row: PgRow| UserId(row.get("user_id")))
        .fetch_one(&state.store.connection)
        .await;

    match find_session {
        Ok(res) => {
            // send the extension to the next request
            info!("{:?}", res);
            request.extensions_mut().insert(res);
            Ok((jar, next.run(request).await))
        }
        Err(_) => {
            let res = StatusCode::UNAUTHORIZED.into_response();
            info!("{:?}", res);
            return Err(res);
        }
    }
}

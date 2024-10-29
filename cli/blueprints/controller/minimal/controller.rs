use crate::{error::Error, state::AppState};
use axum::{extract::State, http::StatusCode};
use tracing::info;

#[axum::debug_handler]
pub async fn action(State(app_state): State<AppState>) -> Result<StatusCode, Error> {
    todo!("implement!");

    info!("responding with {:?}", StatusCode::OK);

    Ok(StatusCode::OK)
}

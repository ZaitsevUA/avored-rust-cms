use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{extract::State, Json, response::IntoResponse};

pub async fn all_pages_cms_api_handler(
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - all_pages_cms_api_handler", "HANDLER");
    let page_model = state
        .page_service
        .all(&state.db)
        .await?;

    Ok(Json(page_model))
}

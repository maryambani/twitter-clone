use axum::{async_trait, response::IntoResponse, extract::State, Json};
use serde::Deserialize;

use crate::{extractor::DbConnection, AppState, error::ApiResult};

pub mod user;

#[async_trait]
pub trait PublicApiRequest {
    type Response: IntoResponse;
    async fn process_request(
        self,
        conn: DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

pub async fn with_public_handler<'a, Req>(
    conn: DbConnection,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where
    Req: PublicApiRequest + Deserialize<'a>,
{
    payload.process_request(conn, state).await
}
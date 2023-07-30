use axum::{Json, async_trait};
use chrono::{Duration, Utc, DateTime};
use hyper::StatusCode;
use tracing::info;
use uchat_endpoint::{user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk}, post::endpoint::{NewPostOk, NewPost}};
use uchat_query::{session::Session, post::Post};
use uchat_domain::ids::*;

use crate::{extractor::{DbConnection, UserSession}, AppState, error::ApiResult};

use super::AuthorizedApiRequest;



#[async_trait]
impl AuthorizedApiRequest for NewPost {
    type Response = (StatusCode, Json<NewPostOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let post = Post::new(session.user_id, self.content, self.options)?;

        let post_id = uchat_query::post::new(&mut conn, post)?;

        Ok((StatusCode::OK, Json(NewPostOk { post_id})))
    }
}

use ::axum::{extract::State, http::StatusCode, response::IntoResponse};
use ::serde::Deserialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*, repositories::account::AccountRepository, services::middleware::ValidatedJson,
};
use crate::services::middleware::Claims;


#[utoipa::path(
    delete,
    path = "/v1/account/wipeout",
    tag = super::TAG_ACCOUNT,
    request_body(
        description = "Account wipeout endpoint",
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer access authorization token"),
    ),    
    responses(
        (
            status = OK,
            description = "The Account is wiped out successfully",
        ),
        (
            status = UNAUTHORIZED,
            description = "The Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
pub async fn wipeout(
    State(state): State<Arc<AppState>>,
    claims: Claims<'_>,
) -> Result<impl IntoResponse> {
    claims.auth.unwrap().id;
    


    Ok(StatusCode::CREATED)
}

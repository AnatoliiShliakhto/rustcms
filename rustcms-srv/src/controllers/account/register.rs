use ::axum::{extract::State, http::StatusCode, response::IntoResponse};
use ::serde::Deserialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*, repositories::account::AccountRepository, services::middleware::ValidatedJson,
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct AccountRegistrationPayload<'a> {
    #[validate(email)]
    pub login: Cow<'a, str>,
    #[validate(length(min = 4, max = 30))]
    pub password: Cow<'a, str>,
    #[validate(must_match(other = "password"))]
    pub password_confirmation: Cow<'a, str>,
}

#[utoipa::path(
    post,
    path = "/v1/account/registration",
    tag = super::TAG_ACCOUNT,
    request_body(
        description = "Account registration endpoint",
        content = AccountRegistrationPayload,
    ),
    responses(
        (
            status = CREATED,
            description = "Account is created successfully",
        ),
        (
            status = BAD_REQUEST,
            description = "The Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
pub async fn registration(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<AccountRegistrationPayload<'_>>,
) -> Result<impl IntoResponse> {
    state
        .db
        .create_account(payload.login, Some(payload.password))
        .await?;

    Ok(StatusCode::CREATED)
}

use ::axum::extract::State;
use ::serde::Deserialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{app::*, repositories::accounts::AccountsRepository, services::middleware::*};

#[derive(Deserialize, ToSchema, Validate)]
pub struct AccountRegisterPayload<'a> {
    #[validate(email(message = "Invalid email"))]
    pub login: Cow<'a, str>,
    #[validate(length(min = 4, max = 30, message = "Password must be between 4 and 30 characters"))]
    pub password: Cow<'a, str>,
    #[validate(must_match(other = "password", message = "Passwords don't match"))]
    pub password_confirmation: Cow<'a, str>,
}

#[utoipa::path(
    post,
    path = "/v1/account",
    tag = super::TAG_ACCOUNT,
    request_body(
        description = "Account registration",
        content = AccountRegisterPayload,
    ),
    responses(
        (
            status = OK,
            description = "Account was created",
        ),
        (
            status = BAD_REQUEST,
            description = "Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
#[handler(result)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<AccountRegisterPayload<'_>>,
) {
    state
        .database
        .create_account(payload.login, Some(payload.password))
        .await?;
}

use ::axum::extract::State;
use ::serde::Deserialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*, repositories::account::AccountRepository, services::middleware::ValidatedJson,
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct AccountRegisterPayload<'a> {
    #[validate(email)]
    pub login: Cow<'a, str>,
    #[validate(length(min = 4, max = 30))]
    pub password: Cow<'a, str>,
    #[validate(must_match(other = "password"))]
    pub password_confirmation: Cow<'a, str>,
}

#[utoipa::path(
    post,
    path = "/v1/account/register",
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
        .db
        .create_account(payload.login, Some(payload.password))
        .await?;
}

use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthState<'a> {
    pub id: Cow<'a, str>,
    pub roles: Option<Vec<Cow<'a, str>>>,
}

impl Default for AuthState<'_> {
    fn default() -> Self {
        Self {
            id: Cow::Borrowed(""),
            roles: None,
        }
    }
}

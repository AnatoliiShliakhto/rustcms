use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, collections::HashSet};
use ::utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthState<'a> {
    pub id: Cow<'a, str>,
    pub roles: Option<HashSet<Cow<'a, str>>>,
}

impl Default for AuthState<'_> {
    fn default() -> Self {
        Self {
            id: Cow::Borrowed("anonymous"),
            roles: None,
        }
    }
}

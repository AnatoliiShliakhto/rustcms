use std::borrow::Cow;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PermissionCapabilities<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub roles: Option<HashSet<Cow<'a, str>>>,
}

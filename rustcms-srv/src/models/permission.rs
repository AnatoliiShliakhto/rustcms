use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::utoipa::ToSchema;

use crate::models::Metadata;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Permission<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub custom: bool,
    pub metadata: Metadata<'a>,
}

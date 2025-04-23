use ::chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Metadata<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Cow<'a, str>>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Cow<'a, str>>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}

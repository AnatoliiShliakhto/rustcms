use ::std::borrow::Cow;
use ::serde::{Deserialize, Serialize};
use ::utoipa::ToSchema;

use crate::models::{Metadata, PermissionCapabilities};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Role<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub permissions: Vec<PermissionCapabilities<'a>>,
    pub metadata: Metadata<'a>,
}

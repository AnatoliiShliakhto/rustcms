use ::serde::{Serialize, Deserialize};
use ::std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct PermissionCapabilities<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub create: bool,
    pub view: bool,
    pub edit: bool,
    pub delete: bool,
}

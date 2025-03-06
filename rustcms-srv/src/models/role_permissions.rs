use ::serde::{Serialize, Deserialize};
use ::std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone)]
pub struct RolePermissions<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub permissions: Vec<Cow<'a, str>>,
}
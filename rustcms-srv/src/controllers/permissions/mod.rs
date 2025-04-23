mod create;
mod delete;
mod list;
mod update;

static TAG_PERMISSIONS: &str = "Permissions";

pub use self::{create::*, delete::*, list::*, update::*};

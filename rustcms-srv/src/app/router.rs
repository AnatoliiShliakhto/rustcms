use ::axum::Router;
use ::std::sync::Arc;

use crate::app::*;

use super::api::api_init;

pub async fn app_init(state: &Arc<AppState>) -> Result<Router> {
    Ok(Router::new().merge(api_init(state).await?))
}

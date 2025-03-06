use ::axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use ::serde::de::DeserializeOwned;
use ::validator::Validate;

use crate::app::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        
        value.validate()?;
        
        Ok(ValidatedJson(value))
    }
}

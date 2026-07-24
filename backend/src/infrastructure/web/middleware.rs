use axum::{
    extract::{FromRequest, Json, rejection::JsonRejection},
    http::Request,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::domain::error::Error;

/// A `Json` extractor that runs `validator` on the decoded body, so handlers
/// only ever see a payload that passed its field rules.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value
            .validate()
            .map_err(|err| Error::validation(err.to_string()))?;
        Ok(ValidatedJson(value))
    }
}

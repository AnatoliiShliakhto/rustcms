use ::reqwest::{Error, Response};
use ::serde::{de::DeserializeOwned, Deserialize};
use ::serde_json::Value;
use ::dioxus::{logger::tracing::error, prelude::*};

use crate::alert_dialog;

pub trait ResponseService {
    async fn json<T: DeserializeOwned>(self) -> Option<T>;
    async fn check(self) -> bool;
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    pub message: String,
    pub details: Option<Value>,
}

impl ResponseService for Result<Response, Error> {
    async fn json<T: DeserializeOwned>(self) -> Option<T> {
        let Ok(response) = self else {
            alert_dialog!("error-network-caption", "error-network-text");
            return None;
        };

        if !response.status().is_success() {
            process_error(response).await;
            None
        } else if let Ok(result) = response.json::<T>().await {
            Some(result)
        } else {
            alert_dialog!("error-deserialize-response-caption", "error-deserialize-response-text");
            None
        }
    }

    async fn check(self) -> bool {
        let Ok(response) = self else {
            alert_dialog!("error-network-caption", "error-network-text");
            return false;
        };
        if !response.status().is_success() {
            process_error(response).await;
            false
        } else {
            true
        }
    }
}

async fn process_error(response: Response) {
    let (caption, message) = match response.status().as_u16() {
        400 => ("error-bad-request-caption", "error-bad-request-text"),
        401 => ("error-unauthorized-caption", "error-unauthorized-text"),
        402 => ("error-access-forbidden-caption", "error-access-forbidden-text"),
        422 => ("error-invalid-payload-caption", "error-invalid-payload-text"),
        500 => ("error-internal-server-caption", "error-internal-server-text"),
        _ => ("error-unknown-caption", "error-unknown-text"),
    };

    if let Ok(error) = response.json::<ErrorResponse>().await {
        error!("{error:#?}");
    };

    alert_dialog!(caption, message);
}

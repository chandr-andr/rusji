use std::result;

use reqwest::Error as reqError;
use serde_json::Error as serdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RusjiError {
    #[error("Can't make a request. Check you connection")]
    RequestError(#[from] reqError),

    #[error("Can't serialize incoming data")]
    SerializeError(#[from] serdError),
}

pub type RusjiResult<T> = result::Result<T, RusjiError>;

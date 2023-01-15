use thiserror::Error;
use reqwest::Error as reqError;

#[derive(Error, Debug)]
pub enum RusjiError {
    #[error("Can't make a request. Check you connection")]
    RequestError(#[from] reqError),
}
use core::fmt;
use std::str::FromStr;

pub use error_code_derive::ToErrorInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorInfo<T> {
    #[serde(skip)]
    pub app_code: T, // could be HTTP 400 bad request
    pub code: &'static str, // something like "01E739"
    #[serde(rename = "msg")]
    pub client_msg: &'static str,
    #[serde(skip)]
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Self {
        Self {
            app_code: T::from_str(app_code).expect("Invalid app_code"),
            code,
            client_msg,
            server_msg: server_msg.to_string(),
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            &self.server_msg
        } else {
            self.client_msg
        }
    }
}

// Display: for client facing error message
impl<T> fmt::Display for ErrorInfo<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.client_msg())
    }
}

// Debug: for server log
impl<T> fmt::Debug for ErrorInfo<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.server_msg)
    }
}

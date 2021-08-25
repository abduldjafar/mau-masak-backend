use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub code: i32,
    pub is_error: bool,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, code: i32,is_error: bool, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            code,
            is_error,
            data,
        }
    }
}
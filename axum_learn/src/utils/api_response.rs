/// API Response
///
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u16, message: String, data: Option<T>) -> Self {
        ApiResponse { code, message, data }
    }

    pub fn success(data: T) -> Self {
        ApiResponse {
            code: StatusCode::OK.as_u16(),
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_without_data() -> Self {
        ApiResponse {
            code: StatusCode::OK.as_u16(),
            message: "Success".to_string(),
            data: None,
        }
    }

    pub fn fail(data: T) -> Self {
        ApiResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: "Fail".to_string(),
            data: Some(data),
        }
    }

    pub fn fail_without_data() -> Self {
        ApiResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: "Fail".to_string(),
            data: None,
        }
    }

    pub fn fail_with_message(message: String, data: T) -> Self {
        ApiResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message,
            data: Some(data),
        }
    }

    pub fn error(data: T) -> Self {
        ApiResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Error".to_string(),
            data: Some(data),
        }
    }

    pub fn error_without_data() -> Self {
        ApiResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Error".to_string(),
            data: None,
        }
    }

    pub fn error_with_message(message: String, data: T) -> Self {
        ApiResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message,
            data: Some(data),
        }
    }
}

#[macro_export]
macro_rules! success {
    () => {
        ApiResponse::success_without_data()
    };
    ($data:expr) => {
        ApiResponse::success($data)
    };
    ($code:expr, $message:expr) => {
        ApiResponse::new($code, $message, None)
    };
    ($code:expr, $message:expr, $data:expr) => {
        ApiResponse::new($code, $message, Some($data))
    };
}

#[macro_export]
macro_rules! fail {
    () => {
        ApiResponse::fail_without_data()
    };
    ($data:expr) => {
        ApiResponse::fail($data)
    };
    ($message:expr, $data:expr) => {
        ApiResponse::fail_with_message($message, $data)
    };
    ($code:expr, $message:expr, $data:expr) => {
        ApiResponse::new($code, $message, Some($data))
    };
}

#[macro_export]
macro_rules! error {
    () => {
        ApiResponse::error_without_data()
    };
    ($data:expr) => {
        ApiResponse::error($data)
    };
    ($message:expr, $data:expr) => {
        ApiResponse::error_with_message($message, $data)
    };
    ($code:expr, $message:expr, $data:expr) => {
        ApiResponse::new($code, $message, Some($data))
    };
}

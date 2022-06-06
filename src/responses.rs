// 🐻‍❄️🌧️ Noelware Telemetry: Telemetry project for Noelware, to capture anonymous data about the running products.
// Copyright 2022 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize + Debug,
{
    success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<Error>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty {}

impl Error {
    fn new(code: &str, message: &str) -> Error {
        Error {
            code: code.into(),
            message: message.into(),
        }
    }

    fn from_error<E>(error: E) -> Error
    where
        E: std::error::Error,
    {
        Error::new("UNKNOWN_EXCEPTION", error.to_string().as_str())
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize + Debug,
{
    pub fn new(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            errors: None,
        }
    }
}

pub fn respond<S>(data: S) -> ApiResponse<S>
where
    S: Serialize + Debug,
{
    ApiResponse::new(data)
}

#[allow(dead_code)]
pub fn from_error<E>(error: E) -> ApiResponse<Empty>
where
    E: std::error::Error,
{
    ApiResponse {
        success: false,
        data: None,
        errors: Some(vec![Error::from_error(error)]),
    }
}

#[allow(dead_code)]
pub fn error(code: &str, message: &str) -> ApiResponse<Empty> {
    ApiResponse {
        success: false,
        data: None,
        errors: Some(vec![Error::new(code, message)]),
    }
}

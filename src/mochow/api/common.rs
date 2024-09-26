/*
 * Copyright 2024 Baidu, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 * except in compliance with the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
 * either express or implied. See the License for the specific language governing permissions
 * and limitations under the License.
 */

use serde::Deserialize;
use std::{error::Error, fmt::Display};

use super::ServerErrorCode;

/// CommonResponse, usually used as the response of a request for update info
#[derive(Debug, Clone, Deserialize)]
pub struct CommonResponse {
    /// 0: success, other: error
    pub code: i32,
    /// success or other error message
    pub msg: String,
}

impl Display for CommonResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {},  msg: {}", self.code, self.msg)
    }
}

/// ServiceError, internal error of database service
#[derive(Debug, Clone)]
pub struct ServiceError {
    /// http status code, like 404
    pub status_code: i32,
    pub request_id: String,
    /// the detail error message
    pub resp: CommonResponse,
    pub server_code: ServerErrorCode,
}

impl Error for ServiceError {}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status_code: {}, request_id: {}, msg: {:?}, server_code: {:?}",
            self.status_code, self.request_id, self.resp, self.server_code,
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::mochow::api::{CommonResponse, ServerErrorCode};

    use super::ServiceError;

    #[test]
    fn test_service_err() {
        let err = ServiceError {
            status_code: -1,
            request_id: "12234".to_string(),
            resp: CommonResponse {
                code: 123,
                msg: "test".to_string(),
            },
            server_code: ServerErrorCode::UNKNOWN,
        };
        println!("{}", err)
    }
}

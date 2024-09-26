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

use std::fmt::Display;

use crate::error::SdkError;

#[derive(Debug, Clone)]
pub struct Credentials {
    account: String,
    api_key: String,
    pub(crate) token: String,
}

impl Credentials {
    pub fn new(account: &str, api_key: &str) -> Result<Self, SdkError> {
        if account.is_empty() {
            return Err(SdkError::ParamsError(
                "account should not be empty".to_string(),
            ));
        }
        if api_key.is_empty() {
            return Err(SdkError::ParamsError(
                "api_key should not be empty".to_string(),
            ));
        }
        let token = format!("account={}&api_key={}", account, api_key);
        Ok(Self {
            account: account.to_string(),
            api_key: api_key.to_string(),
            token,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl Display for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "account: {}, api_key: {}", self.account, self.api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_bce_credentials() {
        let credential = Credentials::new("name", "123456").unwrap();
        println!("{}", credential.token);
        println!("{}", credential);
    }
}

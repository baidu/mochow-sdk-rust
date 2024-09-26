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
use derive_builder::Builder;
use reqwest::header::{HeaderValue, USER_AGENT};

/// configuration for the client, account, api_key and endpoint are required
/// ```rust
/// use mochow_rust_sdk::mochow::config::ClientConfigurationBuilder;
/// let config = ClientConfigurationBuilder::default()
/// 	.account("your account")
/// 	.api_key("your api key")
/// 	.endpoint("http://127.0.0.1:5287")
/// 	.build().unwrap();
/// ```
#[derive(Debug, Clone, Builder)]
pub struct ClientConfiguration {
    #[builder(setter(into))]
    pub account: String,
    #[builder(setter(into))]
    pub api_key: String,
    #[builder(setter(into))]
    pub endpoint: String,

    /// version of the API, default is v1, only support v1 now
    #[builder(default = r#""v1".into()"#, setter(skip))]
    pub version: String,

    /// time out in seconds, default is 30 seconds
    #[builder(default = "30", setter(into))]
    pub time_out_seconds: u64,

    /// number of retries, default is 3
    #[builder(default = "3", setter(into))]
    pub max_retries: u32,

    #[builder(default, setter(into))]
    pub user_agent: String,
}

impl ClientConfiguration {
    pub fn get_request_headers(&self) -> Vec<(reqwest::header::HeaderName, HeaderValue)> {
        let mut headers = Vec::new();
        let user_agent = if self.user_agent.is_empty() {
            "mochow-sdk-rust".to_string()
        } else {
            format!("mochow-sdk-rust/{}", self.user_agent)
        };
        headers.push((USER_AGENT, HeaderValue::from_str(&user_agent).unwrap()));
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::ClientConfigurationBuilder;

    #[test]
    fn t_cofig() {
        let conf = ClientConfigurationBuilder::default().build().unwrap();
        println!("{:?}", conf.get_request_headers())
    }
}

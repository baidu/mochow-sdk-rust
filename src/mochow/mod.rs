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
pub mod api;

#[allow(dead_code)]
pub mod client;

pub mod config;

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref TESTACCOUNT: String = "your_account".to_string();
    pub static ref TESTPASSWORD: String = "your_password".to_string();
    pub static ref TESTENDPOINT: String = "http://127.0.0.1:5288".to_string();
    pub static ref TESTDATABSE: String = "book".to_string();
    pub static ref TESTTABLE: String = "book_segments".to_string();
    pub static ref UTCLIENT: crate::mochow::client::MochowClient = crate::mochow::client::MochowClient::new(&TESTACCOUNT, &TESTPASSWORD, &TESTENDPOINT).unwrap();
}

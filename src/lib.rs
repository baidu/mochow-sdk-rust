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

//! ## Example
//!
//! ```
//! use mochow_rust_sdk::mochow;
//!
//! const ACCOUNT: &str = "your_account";
//! const PASSWORD: &str = "your_password";
//! const ENDPOINT: &str = "http://127.0.0.1:5287";
//!
//! #[tokio::main]
//! async fn main() {
//! 	let client = mochow::client::MochowClient::new(ACCOUNT, PASSWORD, ENDPOINT).unwrap();
//!
//!     let _create_database_resp = client.create_database("book").await.unwrap();
//!
//!     let list_database_resp = client.list_database().await.unwrap();
//!     println!("database list: {:?}", list_database_resp.databases);
//!
//!     let _delete_database_resp = client.drop_database("book").await.unwrap();
//!     println!("delete database resp: {}", _delete_database_resp);
//! }
//! ```
//!
pub mod error;
pub mod mochow;

mod auth;

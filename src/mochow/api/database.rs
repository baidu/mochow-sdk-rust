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
use reqwest_middleware::{ClientWithMiddleware, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::mochow::{client::IntoRequest, config::ClientConfiguration};

/**
 * create database args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Serialize, Builder)]
pub struct CreateDatabaseArgs {
    #[builder(setter(into))]
    pub database: String,
}

/**
 * drop database args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Serialize, Builder)]
pub struct DropDatabaseArgs {
    #[builder(setter(into))]
    pub database: String,
}

/**
 * list databse args, response with [ListDatabaseResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct ListDatabaseArgs {}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDatabaseResponse {
    pub code: i32,
    pub msg: String,

    #[serde(default)]
    pub databases: Vec<String>,
}

impl IntoRequest for CreateDatabaseArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> RequestBuilder {
        let url = format!("{}/{}/database?create", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for DropDatabaseArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> RequestBuilder {
        let url = format!("{}/{}/database", config.endpoint, config.version);
        client.delete(url).json(&self)
    }
}

impl IntoRequest for ListDatabaseArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> RequestBuilder {
        let url = format!("{}/{}/database?list", config.endpoint, config.version);
        client.post(url)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::SdkError;
    use anyhow::Result;

    use super::*;

    use crate::mochow::{TESTDATABSE, UTCLIENT};

    #[test]
    fn create_database_param_serialize() -> Result<()> {
        let param = CreateDatabaseArgsBuilder::default()
            .database("test_db")
            .build()?;

        let json = serde_json::to_value(param)?;
        assert_eq!(
            json,
            serde_json::json!({
                "database":"test_db"
            })
        );
        Ok(())
    }

    #[test]
    fn list_database_response_deserialize() -> Result<()> {
        let data = r#"
            {
                "code": 0,
                "msg": "Success",
                "databases": [
                    "test_db1",
                    "test_db2"
                ]
            }
        "#;
        let v: ListDatabaseResponse = serde_json::from_str(&data)?;
        println!("{:?}", v);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_database() -> Result<()> {
        let _ret = UTCLIENT.create_database(&TESTDATABSE).await;
        match _ret {
            Ok(res) => {
                println!("{:?}", res);
            }
            Err(e) => match e {
                // check service error
                SdkError::ServiceError(e) => {
                    println!("{}", e.request_id);
                    println!("err code: {}", e.resp.code);
                    println!("err msg: {}", e.resp.msg);
                    println!("{}", e.status_code);
                    println!("{:?}", e.server_code);
                    return Err(SdkError::ServiceError(e).into());
                }
                _ => return Err(e.into()),
            },
        }
        // println!("{:?}", _ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_database() -> Result<()> {
        let ret = UTCLIENT.list_database().await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_drop_database() -> Result<()> {
        let ret = UTCLIENT.drop_database(&TESTDATABSE).await?;
        println!("{:?}", ret);
        Ok(())
    }
}

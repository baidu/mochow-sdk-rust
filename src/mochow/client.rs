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

/*
client to connect to mochow server
 */
use std::time::Duration;

use derive_builder::Builder;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use serde::{Deserialize, Serialize};

use crate::{auth::credentials, error::SdkError};

use super::{api::*, config::*};

#[derive(Debug, Clone, Builder)]
pub struct MochowClient {
    /// with important credential information
    #[builder(setter(into))]
    pub(crate) credential: credentials::Credentials,
    /// underlying http client
    #[builder(setter(into))]
    pub(crate) http_client: ClientWithMiddleware,

    #[builder(setter(into))]
    pub configuration: ClientConfiguration,
}

/// every request should imple IntoRequest trait, it's just a rest request for http client
pub trait IntoRequest {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> RequestBuilder;
}

impl MochowClient {
    /// create a new mochow client with account, api_key and endpoint
    /// ```rust
    /// use mochow_rust_sdk::mochow::client::MochowClient;
    /// let client = MochowClient::new("account", "api_key", "endpoint").unwrap();
    /// ```
    pub fn new(account: &str, api_key: &str, endpoint: &str) -> Result<Self, SdkError> {
        Self::new_with_configuration(
            &ClientConfigurationBuilder::default()
                .account(account)
                .api_key(api_key)
                .endpoint(endpoint)
                .build()?,
        )
    }

    // get the http client with config
    fn _http_client(config: &ClientConfiguration) -> ClientWithMiddleware {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(config.max_retries);
        ClientBuilder::new(reqwest::Client::new())
            // Trace HTTP request
            .with(TracingMiddleware::default())
            // Retry
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build()
    }

    /// create a new mochow client with configuration
    /// ```rust
    /// use mochow_rust_sdk::mochow::{config::ClientConfigurationBuilder, client::MochowClient};
    /// let config = ClientConfigurationBuilder::default()
    /// 	.account("account")
    /// 	.api_key("api_key")
    /// 	.endpoint("endpoint")
    /// 	.build().unwrap();
    /// let client = MochowClient::new_with_configuration(&config).unwrap();
    /// ```
    pub fn new_with_configuration(config: &ClientConfiguration) -> Result<Self, SdkError> {
        let mut config = config.clone();
        if config.account.is_empty() || config.api_key.is_empty() || config.endpoint.is_empty() {
            return Err(SdkError::ParamsError(
                "account, apiKey and endpoint missing for creating mochow client".to_string(),
            ));
        }
        let auth = credentials::Credentials::new(&config.account, &config.api_key)?;
        let endpoint = config.endpoint.clone();
        let endpoint = if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
            endpoint
        } else {
            format!("http://{}", endpoint)
        };
        config.endpoint = endpoint;
        let ret = MochowClientBuilder::default()
            .credential(auth)
            .http_client(Self::_http_client(&config))
            .configuration(config)
            .build()?;
        Ok(ret)
    }

    /// create a database
    /// ```rust
    /// let _ = client.create_database("test").await?;
    /// ```
    pub async fn create_database(&self, data_base: &str) -> Result<CommonResponse, SdkError> {
        let args = CreateDatabaseArgsBuilder::default()
            .database(data_base)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// drop the database you created, before deleting the database, all tables in the database must be deleted in advance
    /// ```rust
    /// let _ = client.drop_database("test").await?;
    /// ```
    pub async fn drop_database(&self, data_base: &str) -> Result<CommonResponse, SdkError> {
        let args = DropDatabaseArgsBuilder::default()
            .database(data_base)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// list current all databases
    /// ```rust
    /// let ret = client.list_database("test").await?;
    /// println!("{:?}", ret.databases);
    /// ```
    pub async fn list_database(&self) -> Result<ListDatabaseResponse, SdkError> {
        let args = ListDatabaseArgsBuilder::default().build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<ListDatabaseResponse>().await?)
    }

    /// check if the database is exist,
    pub async fn hash_database(&self, data_base: &str) -> Result<bool, SdkError> {
        let list_database_resp = self.list_database().await?;
        Ok(list_database_resp
            .databases
            .contains(&data_base.to_string()))
    }

    /// create table
    /// please check the [crate::mochow::api::CreateTableArgs]
    /// ```rust
    /// use mochow_rust_sdk::mochow::api::{CreateTableArgsBuilder, Partition, TableSchema}
    /// let fields = vec![];
    /// let indexes = vec![];
    /// let args = CreateTableArgsBuilder::default()
    ///     .database("test_db")
    ///     .table("test_table")
    ///     .description("basic test".to_string())
    ///     .replication(3 as u32)
    ///     .partition(Partition {
    ///         partition_type: PartitionType::HASH,
    ///         partition_num: 3,
    ///      })
    ///      .schema(TableSchema {
    ///         fields: fields,
    ///         indexes: indexes,
    ///      })
    ///     .build()?;
    /// let create_table_resp = client.create_table(&args).await?;
    /// println!("{:?}", create_table_resp);
    /// ```
    pub async fn create_table(&self, args: &CreateTableArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// drop table
    pub async fn drop_table(
        &self,
        data_base: &str,
        table: &str,
    ) -> Result<CommonResponse, SdkError> {
        let args = DropTableArgsBuilder::default()
            .database(data_base)
            .table(table)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// list table
    pub async fn list_table(&self, data_base: &str) -> Result<ListTableResponse, SdkError> {
        let args = ListTableArgsBuilder::default()
            .database(data_base)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<ListTableResponse>().await?)
    }

    /// has table
    pub async fn has_table(&self, data_base: &str, table: &str) -> Result<bool, SdkError> {
        let list_table = self.list_table(data_base).await?;
        Ok(list_table.tables.contains(&table.to_string()))
    }

    /// descript table
    pub async fn desc_table(
        &self,
        data_base: &str,
        table: &str,
    ) -> Result<DescriptTableResponse, SdkError> {
        let args = DescriptTableArgsBuilder::default()
            .database(data_base)
            .table(table)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<DescriptTableResponse>().await?)
    }

    /// add field for table, currently only supports adding scalar fields
    /// ```rust
    /// use mochow_rust_sdk::mochow::api::{TableSchema, FieldSchemaBuilder, AddFieldArgsBuilder};
    /// let fields = vec![FieldSchemaBuilder::default()
    ///     .field_name("bookAlias")
    ///     .field_type("STRING")
    ///     .build()?];
    /// let args = AddFieldArgsBuilder::default()
    ///     .database("test_db")
    ///     .table("test_table")
    ///     .schema(TableSchema{
    ///         fields: fields,
    ///         indexes: vec![],
    ///     })
    ///     .build()?;
    /// let ret = client.add_field(&args).await?;
    /// ```
    pub async fn add_field(&self, args: &AddFieldArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// show table stats
    pub async fn show_table_stats(
        &self,
        data_base: &str,
        table: &str,
    ) -> Result<StatsTableResponse, SdkError> {
        let args = StatsTableArgsBuilder::default()
            .database(data_base)
            .table(table)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<StatsTableResponse>().await?)
    }

    /// alias table
    /// ```rust
    /// use mochow_rust_sdk::mochow::api::{AliasTableArgsBuilder};
    /// let args = AddFieldArgsBuilder::default()
    ///     .database("test_db")
    ///     .table("test_table")
    ///     .alias("table_alias1")
    ///     .build()?;
    /// let ret = client.add_field(&args).await?;
    /// ```
    pub async fn alias_table(&self, args: &AliasTableArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// unalias table
    /// ```rust
    /// use mochow_rust_sdk::mochow::api::{UnaliasTableArgsBuilder};
    /// let args = AddFieldArgsBuilder::default()
    ///     .database("test_db")
    ///     .table("test_table")
    ///     .alias("table_alias1")
    ///     .build()?;
    /// let ret = client.add_field(&args).await?;
    /// ```
    pub async fn unalias_table(&self, args: &UnaliasTableArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// create index, only support for vector index
    /// ```rust
    /// let indexes = vec![IndexSchemaBuilder::default()
    ///     .index_name("vector_idx")
    ///     .field("vector")
    ///     .index_type(IndexType::HNSW)
    ///     ..metric_type(MetricType::L2)
    ///     .params(VectorIndexParams::HNSW(HNSWIndexParam {
    ///         m: 16,
    ///         ef_construction: 200,
    ///     }))
    ///     .build()?];
    /// let args = CreateIndexArgsBuilder::default()
    ///     .database("test_db")
    ///     .table("test_table")
    ///     .indexes(indexes)
    ///     .build()?;
    /// let ret = client.create_index(&args).await?;
    /// ```
    pub async fn create_index(&self, args: &CreateIndexArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// descript index
    pub async fn desc_index(
        &self,
        data_base: &str,
        table: &str,
        index_name: &str,
    ) -> Result<DescriptIndexResponse, SdkError> {
        let args = DescriptIndexArgsBuilder::default()
            .database(data_base)
            .table(table)
            .index_name(index_name)
            .build()?;
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<DescriptIndexResponse>().await?)
    }

    /// modify vector index info, only support 'autobuild' attribute
    /// ```rust
    /// let index = IndexSchemaBuilder::default()
    ///     .index_name("vector_idx")
    ///     .auto_build(true)
    ///     .auto_build_policy(
    ///         AutoBuildPolicyBuilder::default()
    ///             .policy_type(AutoBuildPolicyType::PERIODICAL)
    ///             .period_in_second(5000 as u64)
    ///             .build()?,
    ///     )
    ///     .build()?;
    /// let args = ModifyIndexArgsBuilder::default()
    ///     .database("test_db")
    ///     .table("test_table")
    ///     .index(index)
    ///     .build()?;
    /// let ret = client.modify_index(&args).await?;
    /// ```
    pub async fn modify_index(&self, args: &ModifyIndexArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// rebuild index, only support for vector index
    pub async fn rebuild_index(
        &self,
        data_base: &str,
        table: &str,
        index_name: &str,
    ) -> Result<CommonResponse, SdkError> {
        let args = RebuildIndexArgsBuilder::default()
            .database(data_base)
            .table(table)
            .index_name(index_name)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// delete index
    pub async fn delete_index(
        &self,
        data_base: &str,
        table: &str,
        index_name: &str,
    ) -> Result<CommonResponse, SdkError> {
        let args = DeleteIndexArgsBuilder::default()
            .database(data_base)
            .table(table)
            .index_name(index_name)
            .build()?;
        let req = self.prepare_request(args);
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// insert row, when the primary key of the record already exists, an insertion error occurs, not support insert batch atomicity
    /// ```rust
    /// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    /// struct MyRecord {
    ///     #[serde(default)]
    ///     id: String,
    ///     #[serde(default, rename = "bookName")]
    ///     book_name: String,
    ///     #[serde(default)]
    ///     author: String,
    ///     #[serde(default)]
    ///     page: i32,
    ///     #[serde(default)]
    ///     vector: Vec<f64>,
    /// }
    /// // insert row with a defined struct
    /// let args1 = InsertRowArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .rows(vec![MyRecord {
    ///         id: "0001".to_string(),
    ///         book_name: "西游记".to_string(),
    ///         author: "吴承恩".to_string(),
    ///         page: 21,
    ///         vector: vec![0.2123, 0.24, 0.213],
    ///     }])
    ///     .build()?;
    /// // insert row with a json object
    /// let _ret = client.insert_row(&args1).await?;
    /// let args2 = InsertRowArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .rows(vec![serde_json::json!({
    ///         "id": "0002",
    ///         "bookName": "西游记",
    ///         "author": "吴承恩",
    ///         "page": 22,
    ///         "vector": [0.2123, 0.24, 0.213],
    ///     })])
    ///     .build()?;
    /// let _ret = client.insert_row(&args2).await?;
    /// ```
    pub async fn insert_row<T: Serialize + Clone>(
        &self,
        args: &InsertRowArgs<T>,
    ) -> Result<InsertRowsResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<InsertRowsResponse>().await?)
    }

    /// upsert row, when the primary key of the record already exists, overwrite the old data with the new data as a whole, not support insert batch atomicity
    /// ```
    /// let args = UpsertRowArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .rows(vec![
    ///         serde_json::json!({
    ///             "id":       "0001",
    ///             "bookName": "西游记",
    ///             "author":   "吴承恩",
    ///             "page":     21,
    ///             "vector":   [0.2123, 0.21, 0.213],
    ///         })
    ///     ])
    ///     .build()?;
    /// let _ret = client.upsert_row(&args).await?;
    /// ```
    pub async fn upsert_row<T: Serialize + Clone>(
        &self,
        args: &UpsertRowArgs<T>,
    ) -> Result<UpsertRowsResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<UpsertRowsResponse>().await?)
    }

    /// update row, update the value of one or more scalar fields in a specified record
    /// ```rust
    /// let args = UpdateRowArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .primary_key(serde_json::json!({
    ///         "id": "0001",
    ///     }))
    ///     .update(serde_json::json!({
    ///         "bookName": "红楼梦",
    ///         "author":   "曹雪芹",
    ///         "page":     100,
    ///     }))
    ///     .build()?;
    /// let _ret = client.update_row(&args).await?;
    /// ```
    pub async fn update_row(&self, args: &UpdateRowArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// delete rows, you can delete multiple records by primary key, or filter the records to be deleted
    /// ```rust
    /// let args = DeleteRowArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .primary_key(serde_json::json!({
    ///         "id": "0001",
    ///     }))
    ///     .filter("page >= 22")
    ///     .build()?;
    /// let _ret = UTCLIENT.delete_rows(&args).await?;
    /// ```
    pub async fn delete_rows(&self, args: &DeleteRowArgs) -> Result<CommonResponse, SdkError> {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<CommonResponse>().await?)
    }

    /// query row, query single row by primary key
    /// ```rust
    /// let args = QueryRowArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .primary_key(serde_json::json!({
    ///         "id": "0001",
    ///     }))
    ///     .projections(vec!["id".to_string(), "bookName".to_string()])
    ///     .retrieve_vector(false)
    ///     .build()?;
    /// let query_ret: QueryRowsResponse<MyRecord> = UTCLIENT.query_row(&args).await?;
    /// println!("query_row ret: {:?}", query_ret.row);
    /// let row1 = query_ret.row;
    /// let query_ret: QueryRowsResponse<serde_json::Value> = UTCLIENT.query_row(&args).await?;
    /// println!("query_row ret: {:?}", query_ret.row);
    /// // convert json value to struct
    /// let row2 = serde_json::from_value(query_ret.row)?;
    /// assert_eq!(row1, row2);
    /// ```
    pub async fn query_row<T>(&self, args: &QueryRowArgs) -> Result<QueryRowsResponse<T>, SdkError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<QueryRowsResponse<T>>().await?)
    }

    /// search rows
    /// basing ann search of vector fields, support filter by scalar fields
    /// ```rust
    /// let search_args = SearchRowsArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .anns(
    ///         AnnsSearchParamsBuilder::default()
    ///             .vector_field("vector")
    ///             .vector_floats(vec![0.3123, 0.43, 0.213])
    ///             .params(VectorSearchParams::HNSW(HNSWSearchParams {
    ///                 ef: 200,
    ///                 limit: 10,
    ///                 distance_far: None,
    ///                 distance_near: None,
    ///                 pruning: false,
    ///             }))
    ///             .filter("bookName = '三国演义'")
    ///             .build()?,
    ///     )
    ///     .retrieve_vector(true)
    ///     .build()?;
    /// let ret: SearchRowsResponse<serde_json::Value> = client.search_rows(&search_args).await?;
    /// ```
    pub async fn search_rows<T>(
        &self,
        args: &SearchRowsArgs,
    ) -> Result<SearchRowsResponse<T>, SdkError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<SearchRowsResponse<T>>().await?)
    }

    /// select rows
    /// filter records by scalar fields
    /// ```rust
    /// let mut args = SelectRowsArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .projections(vec![
    ///         "id".to_string(),
    ///         "bookName".to_string(),
    ///         "page".to_string(),
    ///     ])
    ///     .filter("page > 21")
    ///     .limit(1 as u32)
    ///     .build()?;
    /// loop {
    ///     let ret: SelectRowsResponse<serde_json::Value> = client.select_rows(&args).await?;
    ///     println!("select_rows ret: {:?}", ret);
    ///     if !ret.is_truncated {
    ///         break;
    ///     } else {
    ///         args.marker = Some(ret.next_marker);
    ///     }
    /// }
    /// ```
    pub async fn select_rows<T>(
        &self,
        args: &SelectRowsArgs,
    ) -> Result<SelectRowsResponse<T>, SdkError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<SelectRowsResponse<T>>().await?)
    }

    /// batch search rows
    /// 1. basing ann search of vector fields, support filter by scalar fields
    /// 2. support batch query
    /// ```rust
    /// let batch_ann_params = BatchAnnsSearchParamsBuilder::default()
    ///     .vector_field("vector")
    ///     .vector_floats(vec![vec![0.3123, 0.43, 0.213], vec![0.5512, 0.33, 0.43]])
    ///     .params(VectorSearchParams::HNSW(HNSWSearchParams {
    ///         ef: 200,
    ///         limit: 10,
    ///         distance_far: None,
    ///         distance_near: None,
    ///         pruning: false,
    ///     }))
    ///     .filter("bookName = '三国演义'")
    ///     .build()?;
    /// let batch_search_args = BatchSearchRowsArgsBuilder::default()
    ///     .database(&TESTDATABSE.to_string())
    ///     .table(&TESTTABLE.to_string())
    ///     .anns(batch_ann_params)
    ///     .retrieve_vector(true)
    ///     .build()?;
    /// let batch_rets: BatchSearchRowsResponse<serde_json::Value> =
    ///     UTCLIENT.batch_search_rows(&batch_search_args).await?;
    /// for (i, bs) in batch_rets.results.iter().enumerate() {
    ///     println!("batch: {}, {:?}", i, bs.search_vector_floats);
    ///     for (j, ss) in bs.rows.iter().enumerate() {
    ///         println!("{}, {:?}", j, ss);
    ///     }
    /// }
    /// ```
    pub async fn batch_search_rows<T>(
        &self,
        args: &BatchSearchRowsArgs,
    ) -> Result<BatchSearchRowsResponse<T>, SdkError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let req = self.prepare_request(args.clone());
        let res = req.send_and_log().await?;
        Ok(res.json::<BatchSearchRowsResponse<T>>().await?)
    }

    fn prepare_request(&self, req: impl IntoRequest) -> RequestBuilder {
        let mut req = req.into_request(&self.configuration, &self.http_client);
        if !self.credential.token.is_empty() {
            req = req.bearer_auth(&self.credential.token)
        };
        for (key, value) in &self.configuration.get_request_headers() {
            req = req.header(key, value);
        }
        req.timeout(Duration::from_secs(self.configuration.time_out_seconds))
    }
}

trait SendAndLog {
    async fn send_and_log(self) -> Result<Response, SdkError>;
}

impl SendAndLog for RequestBuilder {
    async fn send_and_log(self) -> Result<Response, SdkError> {
        let res = self.send().await?;
        let status_code = res.status();
        let request_id = if let Some(header_value) = res.headers().get("Request-ID") {
            if let Ok(request_id) = header_value.to_str() {
                request_id.to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        if status_code.is_client_error() || status_code.is_server_error() {
            // try to parse service error message, if failed, use default error message
            let service_msg = res.json::<CommonResponse>().await;
            let msg = match service_msg {
                Ok(msg) => msg,
                Err(e) => CommonResponse {
                    code: -1,
                    msg: format!("Service json error message decode failed: {}", e),
                },
            };
            return Err(SdkError::ServiceError(ServiceError {
                status_code: status_code.as_u16() as i32,
                request_id: request_id,
                server_code: msg.clone().code.into(),
                resp: msg,
            }));
        }
        Ok(res)
    }
}

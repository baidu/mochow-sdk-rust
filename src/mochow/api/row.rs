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
use serde::{Deserialize, Serialize};

use crate::mochow::{client::IntoRequest, config::ClientConfiguration};

use super::ReadConsistency;

/**
 * insert row args response with [InsertRowsResponse]
 * you can use serde_json::Value as T or any struct that implements Serialize
 */

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InsertRowArgs<T> {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    /// The inserted dataset supports a maximum of 1000 entries per batch
    #[builder(setter(into))]
    pub rows: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsertRowsResponse {
    pub code: i32,
    pub msg: String,
    /// Number of successfully written records
    #[serde(rename = "affectedCount")]
    pub affected_count: i32,
}

/**
 * upsert row args response with [UpsertRowsResponse]
 * you can use serde_json::Value as T or any struct that implements Serialize
 */

#[derive(Debug, Clone, Builder, Serialize)]
pub struct UpsertRowArgs<T> {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    /// The inserted dataset supports a maximum of 1000 entries per batch
    #[builder(setter(into))]
    pub rows: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertRowsResponse {
    pub code: i32,
    pub msg: String,
    /// Number of successfully written records
    #[serde(rename = "affectedCount")]
    pub affected_count: i32,
}

/**
 * update row args response with [crate::mochow::api::CommonResponse]
 */

#[derive(Debug, Clone, Builder, Serialize)]
pub struct UpdateRowArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,

    /// the primary key value of the record to be updated
    #[builder(setter(into))]
    #[serde(rename = "primaryKey")]
    pub primary_key: serde_json::Value,

    /// the partition value of the record to be updated. If the partition key and primary key of the table are the same key, there is no need to fill in the partition key value. The partition key value will only take effect if there is a primary key value
    #[builder(default, setter(strip_option))]
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_ey: Option<serde_json::Value>,

    /// list of fields to be updated and their new values. Updating primary keys, partition keys, and vector fields is not allowed
    #[builder(setter(into))]
    pub update: serde_json::Value,
}

/**
 * delete row args response with [crate::mochow::api::CommonResponse]
 */

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteRowArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<serde_json::Value>,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_ey: Option<serde_json::Value>,

    /// The syntax of the Filter expression is designed based on the WHERE clause syntax of SQL,
    /// [syntax](https://cloud.baidu.com/doc/VDB/s/mlty5lzzb)
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

/**
 * query row args response with [QueryRowsResponse]
 * query single row by primary key
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct QueryRowArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,

    #[builder(setter(into))]
    #[serde(rename = "primaryKey")]
    pub primary_key: serde_json::Value,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<serde_json::Value>,

    /// projection field list, default to empty. When empty, the query result returns all scalar fields by default
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "projections", skip_serializing_if = "Option::is_none")]
    pub projections: Option<Vec<String>>,

    /// whether to return the vector field values in the query result record
    #[builder(default, setter(strip_option))]
    #[serde(rename = "retrieveVector", skip_serializing_if = "Option::is_none")]
    pub retrieve_vector: Option<bool>,

    /// consistency level of query request,
    /// EVENT (default): final consistency,
    /// STRONG: strong Consistency
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "readConsistency", skip_serializing_if = "Option::is_none")]
    pub read_consistency: Option<ReadConsistency>,
}

/// response of query row, you can use serde_json::Value as T or any struct that implements Deserialize
#[derive(Debug, Clone, Deserialize)]
pub struct QueryRowsResponse<T> {
    pub code: i32,
    pub msg: String,
    pub row: T,
}

/**
 * search rows args response with [SearchRowsResponse]
 * basing ann search of vector fields, support filter by scalar fields
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct SearchRowsArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,

    /// query parameter
    #[builder(setter(into))]
    pub anns: AnnsSearchParams,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_ey: Option<serde_json::Value>,

    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "projections", skip_serializing_if = "Option::is_none")]
    pub projections: Option<Vec<String>>,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "retrieveVector", skip_serializing_if = "Option::is_none")]
    pub retrieve_vector: Option<bool>,

    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "readConsistency", skip_serializing_if = "Option::is_none")]
    pub read_consistency: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct AnnsSearchParams {
    #[builder(setter(into))]
    #[serde(rename = "vectorField")]
    pub vector_field: String,

    /// target vector
    #[builder(setter(into))]
    #[serde(rename = "vectorFloats")]
    pub vector_floats: Vec<f64>,

    #[builder(setter(into))]
    pub params: VectorSearchParams,

    /// SQL [syntax](https://cloud.baidu.com/doc/VDB/s/mlty5lzzb)
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum VectorSearchParams {
    FLAT(FLATSearchParams),
    HNSW(HNSWSearchParams),
    HNSWPQ(HNSWPQSearchParams),
    PUCK(PUCKSearchParams),
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct HNSWSearchParams {
    /// the size of the dynamic candidate list in the HNSW algorithm retrieval process
    #[builder(setter(into))]
    pub ef: u32,
    #[builder(default = "50", setter(into))]
    pub limit: u32,

    /// the farthest distance in the range retrieval scenario
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceFar", skip_serializing_if = "Option::is_none")]
    pub distance_far: Option<f64>,

    /// the nearest distance in the range retrieval scenario
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceNear", skip_serializing_if = "Option::is_none")]
    pub distance_near: Option<f64>,

    #[builder(default, setter(into))]
    pub pruning: bool,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct HNSWPQSearchParams {
    /// the size of the dynamic candidate list in the HNSWPQ algorithm retrieval process
    #[builder(setter(into))]
    pub ef: u32,
    #[builder(default = "50", setter(into))]
    pub limit: u32,

    /// the farthest distance in the range retrieval scenario
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceFar", skip_serializing_if = "Option::is_none")]
    pub distance_far: Option<f64>,

    /// the nearest distance in the range retrieval scenario
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceNear", skip_serializing_if = "Option::is_none")]
    pub distance_near: Option<f64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct PUCKSearchParams {
    /// the size of the coarse clustering center candidate set in the PUCK algorithm retrieval process
    #[builder(setter(into))]
    #[serde(rename = "searchCoarseCount")]
    pub search_coarse_count: u32,
    #[builder(default = "50", setter(into))]
    pub limit: u32,
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceFar", skip_serializing_if = "Option::is_none")]
    pub distance_far: Option<f64>,
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceNear", skip_serializing_if = "Option::is_none")]
    pub distance_near: Option<f64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct FLATSearchParams {
    #[builder(default = "50", setter(into))]
    pub limit: u32,

    /// the farthest distance in the range retrieval scenario
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceFar", skip_serializing_if = "Option::is_none")]
    pub distance_far: Option<f64>,

    /// the nearest distance in the range retrieval scenario
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "distanceNear", skip_serializing_if = "Option::is_none")]
    pub distance_near: Option<f64>,
}

/// response of search row, you can use serde_json::Value as T or any struct that implements Deserialize
#[derive(Debug, Clone, Deserialize)]
pub struct SearchRowsResponse<T> {
    pub code: i32,
    pub msg: String,
    pub rows: Vec<RowResult<T>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RowResult<T> {
    /// record result
    pub row: T,
    /// the distance of this record from the target vector
    pub distance: f64,
    /// the higher the score, the more similar it is to the target vector
    pub score: f64,
}

/**
 * select rows args response with [SelectRowsResponse]
 * filter records by scalar fields
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct SelectRowsArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,

    /// SQL [syntax](https://cloud.baidu.com/doc/VDB/s/mlty5lzzb)
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// starting point for pagination of queries, default from the first eligible record
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<serde_json::Value>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "projections", skip_serializing_if = "Option::is_none")]
    pub projections: Option<Vec<String>>,

    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "readConsistency", skip_serializing_if = "Option::is_none")]
    pub read_consistency: Option<String>,
}

/// response of select row, you can use serde_json::Value as T or any struct that implements Deserialize
#[derive(Debug, Clone, Deserialize)]
pub struct SelectRowsResponse<T> {
    pub code: i32,
    pub msg: String,
    pub rows: Vec<T>,

    #[serde(rename = "isTruncated")]
    pub is_truncated: bool,

    #[serde(rename = "nextMarker")]
    pub next_marker: serde_json::Value,
}

/**
 * batch search rows args response with [BatchSearchRowsResponse],
 * basing ann search of vector fields, support filter by scalar fields,
 * you can input multiple vectors to search, and get result for every vector
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct BatchSearchRowsArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    pub anns: BatchAnnsSearchParams,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_ey: Option<serde_json::Value>,

    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "projections", skip_serializing_if = "Option::is_none")]
    pub projections: Option<Vec<String>>,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "retrieveVector", skip_serializing_if = "Option::is_none")]
    pub retrieve_vector: Option<bool>,

    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "readConsistency", skip_serializing_if = "Option::is_none")]
    pub read_consistency: Option<String>,
}

/*
 * batch search rows params is similiar to search rows params,
 * the difference is that queries can be launched on multiple vectors
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct BatchAnnsSearchParams {
    #[builder(setter(into))]
    #[serde(rename = "vectorField")]
    pub vector_field: String,

    /// target vector list
    #[builder(setter(into))]
    #[serde(rename = "vectorFloats")]
    pub vector_floats: Vec<Vec<f64>>,

    #[builder(setter(into))]
    pub params: VectorSearchParams,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchSearchRowsResponse<T> {
    pub code: i32,
    pub msg: String,
    pub results: Vec<BatchRowResult<T>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchRowResult<T> {
    /// related target vector
    #[serde(default, rename = "searchVectorFloats")]
    pub search_vector_floats: Vec<f64>,
    pub rows: Vec<RowResult<T>>,
}

impl<T: Serialize> IntoRequest for InsertRowArgs<T> {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?insert", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl<T: Serialize> IntoRequest for UpsertRowArgs<T> {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?upsert", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for UpdateRowArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?update", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for DeleteRowArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?delete", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for QueryRowArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?query", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for SearchRowsArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?search", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for SelectRowsArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?select", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for BatchSearchRowsArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &reqwest_middleware::ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/row?batchSearch", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mochow::{TESTDATABSE, TESTTABLE, UTCLIENT};
    use anyhow::Result;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct MyRecord {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "bookName")]
        book_name: String,
        #[serde(default)]
        author: String,
        #[serde(default)]
        page: i32,
        #[serde(default)]
        vector: Vec<f64>,
    }

    #[test]
    fn test_insert_row_serialize() -> Result<()> {
        let args = InsertRowArgsBuilder::default()
            .database("test_db")
            .table("test_table")
            .rows(vec![serde_json::json!(
                {
                    "id": "00001",
                    "username": "alice",
                    "vector_field": [
                        0.2323234,
                        0.34534545,
                        0.9837234
                    ]
                }
            )])
            .build()?;
        let json = serde_json::to_string(&args)?;
        println!("{}", json);
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_row() -> Result<()> {
        // insert row with a defined struct
        let args1 = InsertRowArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .rows(vec![MyRecord {
                id: "0001".to_string(),
                book_name: "西游记".to_string(),
                author: "吴承恩".to_string(),
                page: 21,
                vector: vec![0.2123, 0.24, 0.213],
            }])
            .build()?;
        let _ret = UTCLIENT.insert_row(&args1).await?;
        println!("insert_row ret: {:?}", _ret);

        // insert row with a json object
        let args2 = InsertRowArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .rows(vec![serde_json::json!({
                "id": "0002",
                "bookName": "西游记",
                "author": "吴承恩",
                "page": 22,
                "vector": [0.2123, 0.24, 0.213],
            })])
            .build()?;
        let _ret = UTCLIENT.insert_row(&args2).await?;
        println!("insert_row ret: {:?}", _ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_row() -> Result<()> {
        let args = UpsertRowArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .rows(vec![
                serde_json::json!({
                    "id":       "0001",
                    "bookName": "西游记",
                    "author":   "吴承恩",
                    "page":     21,
                    "vector":   [0.2123, 0.21, 0.213],
                }),
                serde_json::json!({
                    "id":       "0002",
                    "bookName": "西游记",
                    "author":   "吴承恩",
                    "page":     22,
                    "vector":   [0.2123, 0.22, 0.213],
                }),
                serde_json::json!({
                    "id":       "0003",
                    "bookName": "三国演义",
                    "author":   "罗贯中",
                    "page":     23,
                    "vector":   [0.2123, 0.23, 0.213],
                }),
                serde_json::json!({
                    "id":       "0004",
                    "bookName": "三国演义",
                    "author":   "罗贯中",
                    "page":     24,
                    "vector":   [0.2123, 0.24, 0.213],
                }),
            ])
            .build()?;
        let _ret = UTCLIENT.upsert_row(&args).await?;
        println!("upsert_row ret: {:?}", _ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_row() -> Result<()> {
        let args = UpdateRowArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .primary_key(serde_json::json!({
                "id": "0001",
            }))
            .update(serde_json::json!({
                "bookName": "红楼梦",
                "author":   "曹雪芹",
                "page":     100,
            }))
            .build()?;
        let _ret = UTCLIENT.update_row(&args).await?;
        println!("update_row ret: {:?}", _ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_row() -> Result<()> {
        let args = DeleteRowArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            // .primary_key(serde_json::json!({
            //     "id": "0001",
            // }))
            .filter("page >= 22")
            .build()?;
        let _ret = UTCLIENT.delete_rows(&args).await?;
        println!("delete_row ret: {:?}", _ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_row() -> Result<()> {
        let args = QueryRowArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .primary_key(serde_json::json!({
                "id": "0001",
            }))
            .projections(vec!["id".to_string(), "bookName".to_string()])
            .retrieve_vector(false)
            .build()?;
        let query_ret: QueryRowsResponse<MyRecord> = UTCLIENT.query_row(&args).await?;
        println!("query_row ret: {:?}", query_ret.row);
        let row1 = query_ret.row;
        let query_ret: QueryRowsResponse<serde_json::Value> = UTCLIENT.query_row(&args).await?;
        println!("query_row ret: {:?}", query_ret.row);
        // convert json value to struct
        let row2 = serde_json::from_value(query_ret.row)?;
        assert_eq!(row1, row2);
        Ok(())
    }

    #[tokio::test]
    async fn test_select_row() -> Result<()> {
        let mut args = SelectRowsArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .projections(vec![
                "id".to_string(),
                "bookName".to_string(),
                "page".to_string(),
            ])
            .filter("page > 21")
            .limit(1 as u32)
            .build()?;
        loop {
            let ret: SelectRowsResponse<serde_json::Value> = UTCLIENT.select_rows(&args).await?;
            println!("select_rows ret: {:?}", ret);
            if !ret.is_truncated {
                break;
            } else {
                args.marker = Some(ret.next_marker);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_search() -> Result<()> {
        let search_args = SearchRowsArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .anns(
                AnnsSearchParamsBuilder::default()
                    .vector_field("vector")
                    .vector_floats(vec![0.3123, 0.43, 0.213])
                    .params(VectorSearchParams::HNSW(HNSWSearchParams {
                        ef: 200,
                        limit: 10,
                        distance_far: None,
                        distance_near: None,
                        pruning: false,
                    }))
                    .filter("bookName = '三国演义'")
                    .build()?,
            )
            .retrieve_vector(true)
            .build()?;
        let ret: SearchRowsResponse<serde_json::Value> = UTCLIENT.search_rows(&search_args).await?;
        println!("search_rows ret: {:?}", ret.rows);
        Ok(())
    }

    #[tokio::test]
    async fn test_batch_search() -> Result<()> {
        let batch_ann_params = BatchAnnsSearchParamsBuilder::default()
            .vector_field("vector")
            .vector_floats(vec![vec![0.3123, 0.43, 0.213], vec![0.5512, 0.33, 0.43]])
            .params(VectorSearchParams::HNSW(HNSWSearchParams {
                ef: 200,
                limit: 10,
                distance_far: None,
                distance_near: None,
                pruning: false,
            }))
            .filter("bookName = '三国演义'")
            .build()?;
        let batch_search_args = BatchSearchRowsArgsBuilder::default()
            .database(&TESTDATABSE.to_string())
            .table(&TESTTABLE.to_string())
            .anns(batch_ann_params)
            .retrieve_vector(true)
            .build()?;
        let batch_rets: BatchSearchRowsResponse<serde_json::Value> =
            UTCLIENT.batch_search_rows(&batch_search_args).await?;
        for (i, bs) in batch_rets.results.iter().enumerate() {
            println!("batch: {}, {:?}", i, bs.search_vector_floats);
            for (j, ss) in bs.rows.iter().enumerate() {
                println!("{}, {:?}", j, ss);
            }
        }
        Ok(())
    }
}

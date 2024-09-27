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
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::mochow::{client::IntoRequest, config::ClientConfiguration};

use super::{FieldType, IndexSchema, PartitionType, TableState};

/// click <https://cloud.baidu.com/doc/VDB/s/flrsob0zr> for more details

/**
 * create table args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateTableArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,

    /// the description of table
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// the number of replicas(including primary replica) for a single tablet, range of [1, 10]，
    /// for high availability, it is recommended >= 3,
    /// note: the number of relicas needs to less than or equal to the number of DataNodes
    #[builder(setter(into))]
    pub replication: u32,

    /// The number of partitions in the table, with a value range of \[1,1000\].
    /// Suggest evaluating the number of partitions based on the expected total number of records.
    /// Suggest controlling the number of records in a single partition between 1 million and 10 million.
    #[builder(default, setter(into))]
    pub partition: Partition,

    /// is suppport dynamic field, default is false
    #[builder(default = "Some(false)", setter(strip_option))]
    #[serde(rename = "enableDynamicField", skip_serializing_if = "Option::is_none")]
    pub enable_dynamic_field: Option<bool>,

    /// schema args for table
    #[builder(default, setter(into))]
    pub schema: TableSchema,
}

#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
pub struct Partition {
    /// there is only one "HASH",option currenctly
    #[builder(default, setter(into))]
    #[serde(rename = "partitionType")]
    pub partition_type: PartitionType,

    /// number of tablet for table, range [1, 1000]
    #[builder(default, setter(into))]
    #[serde(rename = "partitionNum")]
    pub partition_num: u32,
}

#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
pub struct TableSchema {
    /// the fields of table
    #[builder(default, setter(into))]
    #[serde(default)]
    pub fields: Vec<FieldSchema>,
    /// the indexes of table
    #[builder(default, setter(into))]
    #[serde(default)]
    pub indexes: Vec<IndexSchema>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct FieldSchema {
    /// must start with a letter,
    /// only lowercase and uppercase letters, numbers and underscores are allowed
    #[builder(default, setter(into))]
    #[serde(default, rename = "fieldName")]
    pub field_name: String,

    /// BOOL、INT8、UINT8、INT16、UINT16、INT32、UINT32、INT64、UINT64、FLOAT、DOUBLE、DATE、DATETIME、TIMESTAMP、UUID、STRING、BINARY、TEXT、TEXT_GBK、TEXT_GB18030、FLOAT_VECTOR
    #[builder(setter(into))]
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,

    /// only a single field is supported for primary key currently,
    /// unsupport field with type: BOOL、FLOAT、DOUBLE、FLOAT_VECTOR
    #[builder(default, setter(into))]
    #[serde(default, rename = "primaryKey")]
    pub primary_key: bool,

    /// only a single field is supported as a partition key, which can be a primary key or not,
    /// a table can only have one partition key, each record will be hashed and mapped to diffrent partition,
    /// unsupport field with type: BOOL、FLOAT、DOUBLE、FLOAT_VECTOR
    #[builder(default, setter(into))]
    #[serde(default, rename = "partitionKey")]
    pub partition_key: bool,

    /// is autoincreament, only applicable to primary key fields of type UINT64
    #[builder(default, setter(into))]
    #[serde(default, rename = "autoIncrement")]
    pub auto_increment: bool,

    /// primary key field, partition key field, secondary index key field and vector field cannot be nullable
    #[builder(default, setter(into))]
    #[serde(default, rename = "notNull")]
    pub not_null: bool,

    /// vector dimension. This parameter needs to be specified only when the data type is FLOAT_VECTOR.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<u32>,
}

/**
 * drop table args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct DropTableArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
}

/**
 * list table args, response with [ListTableResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct ListTableArgs {
    #[builder(setter(into))]
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTableResponse {
    pub code: i32,
    pub msg: String,

    #[serde(default)]
    pub tables: Vec<String>,
}

/**
 * descript table args, response with [DescriptTableResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct DescriptTableArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescriptTable {
    pub database: String,
    pub table: String,

    #[serde(rename = "createTime")]
    pub create_time: String,
    pub description: String,
    pub replication: u32,

    pub partition: Partition,

    #[serde(rename = "enableDynamicField")]
    pub enable_dynamic_field: bool,

    pub state: TableState,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default)]
    pub schema: TableSchema,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescriptTableResponse {
    pub code: i32,
    pub msg: String,

    pub table: DescriptTable,
}

/**
 * add field args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct AddFieldArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    /// schema args for table
    #[builder(default, setter(into))]
    pub schema: TableSchema,
}

/**
 * stats table args, response with [StatsTableResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct StatsTableArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatsTableResponse {
    pub code: i32,
    pub msg: String,

    #[serde(rename = "rowCount")]
    pub row_count: u64,
    #[serde(rename = "memorySizeInByte")]
    pub memory_size_in_byte: u64,
    #[serde(rename = "diskSizeInByte")]
    pub disk_size_in_byte: u64,
}

/**
 * alias table args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct AliasTableArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    pub alias: String,
}

/**
 * unalias table args, response with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct UnaliasTableArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    pub alias: String,
}

impl IntoRequest for CreateTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?create", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for DropTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table", config.endpoint, config.version);
        client.delete(url).json(&self)
    }
}

impl IntoRequest for ListTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?list", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for DescriptTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?desc", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for AddFieldArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?addField", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for StatsTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?stats", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for AliasTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?alias", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for UnaliasTableArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/table?unalias", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::super::{
        AutoBuildPolicyBuilder, HNSWIndexParam, IndexSchemaBuilder, VectorIndexParams,
    };
    use super::*;
    use crate::mochow::api::{AutoBuildPolicyType, IndexType, MetricType};
    use crate::mochow::{TESTDATABSE, TESTTABLE, UTCLIENT};

    #[test]
    fn create_table_args_serialize_test() -> Result<()> {
        let args = CreateTableArgsBuilder::default()
            .database("test_db")
            .table("test_table")
            .description("this is description".to_string())
            .replication(3 as u32)
            .enable_dynamic_field(true)
            .partition(
                PartitionBuilder::default()
                    .partition_num(1 as u32)
                    .partition_type(PartitionType::HASH)
                    .build()?,
            )
            .schema(
                TableSchemaBuilder::default()
                    .fields(vec![FieldSchemaBuilder::default()
                        .field_name("name")
                        .field_type(FieldType::TEXT)
                        .dimension(0)
                        .build()?])
                    .indexes(vec![IndexSchemaBuilder::default()
                        .index_name("name")
                        .index_type(IndexType::HNSW)
                        .params(VectorIndexParams::HNSW(HNSWIndexParam {
                            m: 8,
                            ef_construction: 200,
                        }))
                        .metric_type(MetricType::L2)
                        .auto_build(true)
                        .auto_build_policy(
                            AutoBuildPolicyBuilder::default()
                                .policy_type(AutoBuildPolicyType::PERIODICAL)
                                .build()?,
                        )
                        .build()?])
                    .build()?,
            )
            .build()?;
        let json = serde_json::to_value(args)?;
        assert_eq!(
            json,
            serde_json::json!({
                "database": "test_db",
                "table": "test_table",
                "enableDynamicField": true,
                "replication": 3,
                "description": "this is description",
                "partition": {
                    "partitionType": "HASH",
                    "partitionNum": 1,
                },
                "schema": {
                    "fields": [
                        {
                            "fieldName": "name",
                            "fieldType": "TEXT",
                            "partitionKey": false,
                            "primaryKey": false,
                            "autoIncrement": false,
                            "notNull": false,
                            "dimension": 0,
                        }
                    ],
                    "indexes": [
                        {
                            "autoBuild": true,
                            "autoBuildPolicy":  {
                                "periodInSecond":0,
                                "policyType": "PERIODICAL",
                                "rowCountIncrement":0,
                                "rowCountIncrementRatio": 0.0
                            },
                            "field": "",
                            "indexName": "name",
                            "indexType": "HNSW",
                            "metricType": "L2",
                            "params":  {
                                "M":8,
                                "efConstruction": 200
                            }
                        }
                    ]
                }
            })
        );
        println!("{:?}", json);
        Ok(())
    }

    #[test]
    fn descript_table_response_deserialize_test() -> Result<()> {
        let data = r#"
            {
                "database": "test_db",
                "table": "test_table",
                "createTime": "2024-02-02T12:02:08Z",
                "enableDynamicField": true,
                "state": "NORMAL",
                "replication": 3,
                "description": "this is description",
                "partition": {
                    "partitionType": "HASH",
                    "partitionNum": 1
                },
                "schema": {
                    "fields": [
                        {
                            "fieldName": "name",
                            "fieldType": "TEXT",
                            "partitionKey": false,
                            "primaryKey": false,
                            "autoIncrement": false,
                            "notNull": false,
                            "dimension": 0
                        }
                    ],
                    "indexes": [
                        {
                            "autoBuild": true,
                            "autoBuildPolicy":  {
                                "periodInSecond":0,
                                "policyType": "TIMING",
                                "rowCountIncrement":0,
                                "rowCountIncrementRatio": 0.0,
                                "timing": ""
                            },
                            "field": "",
                            "indexName": "name",
                            "indexType": "HNSW",
                            "metricType": "L2",
                            "state": "NORMAL",
                            "params":  {
                                "M":8,
                                "efConstruction": 20
                            }
                        }
                    ]
                }
            }
        "#;

        let v: DescriptTable = serde_json::from_str(&data)?;
        println!("{:?}", v);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_table() -> Result<()> {
        let fields = vec![
            FieldSchemaBuilder::default()
                .field_name("id")
                .field_type(FieldType::STRING)
                .primary_key(true)
                .partition_key(true)
                .not_null(true)
                .build()?,
            FieldSchemaBuilder::default()
                .field_name("bookName")
                .field_type(FieldType::STRING)
                .not_null(true)
                .build()?,
            FieldSchemaBuilder::default()
                .field_name("author")
                .field_type(FieldType::STRING)
                .build()?,
            FieldSchemaBuilder::default()
                .field_name("page")
                .field_type(FieldType::UINT32)
                .build()?,
            FieldSchemaBuilder::default()
                .field_name("vector")
                .field_type(FieldType::FLOAT_VECTOR)
                .not_null(true)
                .dimension(3)
                .build()?,
        ];
        let indexes = vec![
            IndexSchemaBuilder::default()
                .index_name("book_name_idx")
                .field("bookName")
                .index_type(IndexType::SECONDARY_INDEX)
                .build()?,
            IndexSchemaBuilder::default()
                .index_name("vector_idx")
                .field("vector")
                .index_type(IndexType::HNSW)
                .metric_type(MetricType::L2)
                .params(VectorIndexParams::HNSW(HNSWIndexParam {
                    m: 32,
                    ef_construction: 200,
                }))
                .build()?,
        ];
        let args = CreateTableArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .description("basic test".to_string())
            .replication(3 as u32)
            .partition(Partition {
                partition_type: PartitionType::HASH,
                partition_num: 3,
            })
            .schema(TableSchema {
                fields: fields,
                indexes: indexes,
            })
            .build()?;
        let create_table_resp = UTCLIENT.create_table(&args).await?;
        println!("{:?}", create_table_resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_add_field() -> Result<()> {
        let fields = vec![FieldSchemaBuilder::default()
            .field_name("bookAlias")
            .field_type(FieldType::STRING)
            .build()?];
        let args = AddFieldArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .schema(TableSchema {
                fields: fields,
                indexes: vec![],
            })
            .build()?;
        let ret = UTCLIENT.add_field(&args).await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_desc_table() -> Result<()> {
        let ret = UTCLIENT.desc_table(&TESTDATABSE, &TESTTABLE).await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_table() -> Result<()> {
        let ret = UTCLIENT.list_table(&TESTDATABSE).await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_stats_table() -> Result<()> {
        let ret = UTCLIENT.show_table_stats(&TESTDATABSE, &TESTTABLE).await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_alias_table() -> Result<()> {
        let ret = UTCLIENT
            .alias_table("book", "book_segments", "table_alias1")
            .await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_unalias_table() -> Result<()> {
        let ret = UTCLIENT
            .unalias_table("book", "book_segments", "table_alias1")
            .await?;
        println!("{:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_drop_table() -> Result<()> {
        let ret = UTCLIENT.drop_table(&TESTDATABSE, &TESTTABLE).await?;
        println!("{:?}", ret);
        Ok(())
    }
}

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

use super::{AutoBuildPolicyType, IndexState, IndexType, MetricType};

/**
 * create index args response with [crate::mochow::api::CommonResponse]
 *
 * click <https://cloud.baidu.com/doc/VDB/s/Elrsob14o#%E5%93%8D%E5%BA%94%E5%8F%82%E6%95%B0> for more detail of every param
 */

#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateIndexArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(default, setter(into))]
    #[serde(default)]
    pub indexes: Vec<IndexSchema>,
}

/// index param
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct IndexSchema {
    #[builder(default, setter(into))]
    #[serde(default, rename = "indexName")]
    pub index_name: String,

    /// The currently supported types are as follows:
    /// scalar index: SECONDARY,
    /// vector index: HNSW, FLAT
    #[builder(default, setter(into, strip_option))]
    #[serde(default, rename = "indexType", skip_serializing_if = "Option::is_none")]
    pub index_type: Option<IndexType>,

    /// Distance measurement type of vector index,
    /// L2: Euclidean distance,
    /// IP: Inner product distance,
    /// COSINE: Cosine distance.
    #[builder(default, setter(into, strip_option))]
    #[serde(
        default,
        rename = "metricType",
        skip_serializing_if = "Option::is_none"
    )]
    pub metric_type: Option<MetricType>,

    /// Parameter details of vector index
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<VectorIndexParams>,

    /// The target field name that the index acts on
    #[builder(default, setter(into))]
    pub field: String,

    /// is auto build index
    #[builder(default, setter(into))]
    #[serde(default, rename = "autoBuild")]
    pub auto_build: bool,

    /// index state,
    /// this is not param in create index api, but response from server
    #[builder(setter(skip))]
    #[serde(default, skip_serializing)]
    pub state: Option<IndexState>,

    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        rename = "autoBuildPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_build_policy: Option<AutoBuildPolicy>,

    /// index major version: only increase when manually rebuilding the index is completed,
    /// this is not param in create index api, but response from server
    #[builder(default, setter(skip))]
    #[serde(
        default,
        rename = "indexMajorVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_major_version: Option<u64>,
}

///  auto build index strategy
#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
pub struct AutoBuildPolicy {
    /// the policy type of the index:
    /// timing: regularly build indexes, only build once,
    /// periodical: periodically building indexes,
    /// row_count_increment: build index based on new added rows.
    #[builder(setter(into, strip_option))]
    #[serde(
        default,
        rename = "policyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_type: Option<AutoBuildPolicyType>,

    /// time format: LOCAL(%Y-%m-%d %H:%M:%S) and UTC(%Y-%m-%dT%H:%M:%Z),
    /// only build once when the policyType is 'timing' and it takes effect,
    /// When policyType is 'periodic', the incoming time serves as the start time of the periodic policy.
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub timing: String,

    /// periodInSecond: the interval of periodic building indexes,
    /// build index periodically when policyType is 'periodic'.
    #[builder(default, setter(into))]
    #[serde(default, rename = "periodInSecond")]
    pub period_in_second: u64,

    /// build an index when the number of rows in a tablet increases or decreases by more than rowCountIncrement
    #[builder(default, setter(into))]
    #[serde(default, rename = "rowCountIncrement")]
    pub row_count_increment: u64,

    /// build index once when the percentage of rows added or decreased by the tablet is greater than rowCountIncrementRatio
    #[builder(default, setter(into))]
    #[serde(default, rename = "rowCountIncrementRatio")]
    pub row_count_increment_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorIndexParams {
    /// HNSWPQ index
    HNSWPQ(HNSWPQIndexParam),
    /// HNSW index
    HNSW(HNSWIndexParam),
    /// PUCK index
    PUCK(PUCKIndexParam),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HNSWIndexParam {
    /// range \[4, 128\],
    /// neighboring nodes are connected to each node in the HNSW index graph
    #[serde(rename = "M")]
    pub m: u32,
    /// range \[8, 1024\],
    /// the number of temporary neighbor nodes used during build index
    #[serde(rename = "efConstruction")]
    pub ef_construction: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HNSWPQIndexParam {
    /// range \[4, 128\],
    /// neighboring nodes are connected to each node in the HNSW index graph
    #[serde(rename = "M")]
    pub m: u32,
    /// range \[8, 1024\],
    /// the number of temporary neighbor nodes used during build index
    #[serde(rename = "efConstruction")]
    pub ef_construction: u32,

    /// range \[1, dim\]
    /// the number of quantization subspaces, pq quantization correlation coefficient, and requires NSQ | dim. The larger the NSQ, the finer the quantization
    #[serde(rename = "NSQ")]
    pub nsq: u32,

    /// range \[0.0f, 1.0f\]
    /// the sampling rate of k-means and the total number of pq samples, 10000 + (rowCount - 10000)*sampleRate
    #[serde(rename = "sampleRate")]
    pub sample_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PUCKIndexParam {
    /// range \[1, 5000\],
    /// the number of coarse cluster centers in the index
    #[serde(rename = "coarseClusterCount")]
    pub coarse_cluster_count: u32,
    /// range \[1, 5000\],
    /// the number of fine cluster centers under each coarse cluster center
    #[serde(rename = "fineClusterCount")]
    pub fine_cluster_count: u32,
}

/**
 * descript index args response with [DescriptIndexResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct DescriptIndexArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    #[serde(rename = "indexName")]
    pub index_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescriptIndexResponse {
    pub code: i32,
    pub msg: String,

    pub index: IndexSchema,
}

/**
 * rebuild index args reponse with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct RebuildIndexArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    #[serde(rename = "indexName")]
    pub index_name: String,
}

/**
 * delete index args reponse with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteIndexArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    #[serde(rename = "indexName")]
    pub index_name: String,
}

/**
 * modify index args reponse with [crate::mochow::api::CommonResponse]
 */
#[derive(Debug, Clone, Builder, Serialize)]
pub struct ModifyIndexArgs {
    #[builder(setter(into))]
    pub database: String,
    #[builder(setter(into))]
    pub table: String,
    #[builder(setter(into))]
    pub index: IndexSchema,
}

impl IntoRequest for CreateIndexArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/index?create", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for DescriptIndexArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/index?desc", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for RebuildIndexArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/index?rebuild", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

impl IntoRequest for DeleteIndexArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/index", config.endpoint, config.version);
        client.delete(url).json(&self)
    }
}

impl IntoRequest for ModifyIndexArgs {
    fn into_request(
        self,
        config: &ClientConfiguration,
        client: &ClientWithMiddleware,
    ) -> reqwest_middleware::RequestBuilder {
        let url = format!("{}/{}/index?modify", config.endpoint, config.version);
        client.post(url).json(&self)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::mochow::{TESTDATABSE, TESTTABLE, UTCLIENT};

    use super::*;
    #[tokio::test]
    async fn test_rebuild_index() -> Result<()> {
        let ret = UTCLIENT
            .rebuild_index(&TESTDATABSE, &TESTTABLE, "vector_idx")
            .await?;
        println!("rebuild index: {:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_desc_index() -> Result<()> {
        let ret = UTCLIENT
            .desc_index(&TESTDATABSE, &TESTTABLE, "vector_idx_hnswpq")
            .await?;
        println!("desc index: {:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index() -> Result<()> {
        let ret = UTCLIENT
            .delete_index(&TESTDATABSE, &TESTTABLE, "vector_idx_hnswpq")
            .await?;
        println!("delete index: {:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_index_flat() -> Result<()> {
        let indexes = vec![IndexSchemaBuilder::default()
            .index_name("vector_idx_flat")
            .field("vector")
            .index_type(IndexType::FLAT)
            .metric_type(MetricType::L2)
            .build()?];
        let args = CreateIndexArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .indexes(indexes)
            .build()?;
        let ret = UTCLIENT.create_index(&args).await?;
        println!("create index: {:?}", ret);
        Ok(())
    }

    /// remembder there is no index named 'vertor_idx' in the test table before test
    #[tokio::test]
    async fn test_create_index_hnsw() -> Result<()> {
        let indexes = vec![IndexSchemaBuilder::default()
            .index_name("vector_idx_hnsw")
            .field("vector")
            .index_type(IndexType::HNSW)
            .metric_type(MetricType::L2)
            .params(VectorIndexParams::HNSW(HNSWIndexParam {
                m: 16,
                ef_construction: 200,
            }))
            .build()?];
        let args = CreateIndexArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .indexes(indexes)
            .build()?;
        let ret = UTCLIENT.create_index(&args).await?;
        println!("create index: {:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_index_hnswpq() -> Result<()> {
        let indexes = vec![IndexSchemaBuilder::default()
            .index_name("vector_idx_hnswpq")
            .field("vector")
            .index_type(IndexType::HNSWPQ)
            .metric_type(MetricType::L2)
            .params(VectorIndexParams::HNSWPQ(HNSWPQIndexParam {
                m: 16,
                ef_construction: 200,
                nsq: 3,
                sample_rate: 1.0,
            }))
            .build()?];
        let args = CreateIndexArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .indexes(indexes)
            .build()?;
        let ret = UTCLIENT.create_index(&args).await?;
        println!("create index: {:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_index_puck() -> Result<()> {
        let indexes = vec![IndexSchemaBuilder::default()
            .index_name("vector_idx_puck")
            .field("vector")
            .index_type(IndexType::PUCK)
            .metric_type(MetricType::L2)
            .params(VectorIndexParams::PUCK(PUCKIndexParam {
                coarse_cluster_count: 5,
                fine_cluster_count: 5,
            }))
            .build()?];
        let args = CreateIndexArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .indexes(indexes)
            .build()?;
        let ret = UTCLIENT.create_index(&args).await?;
        println!("create index: {:?}", ret);
        Ok(())
    }

    #[tokio::test]
    async fn test_modify_index() -> Result<()> {
        let index = IndexSchemaBuilder::default()
            .index_name("vector_idx")
            .auto_build(true)
            .auto_build_policy(
                AutoBuildPolicyBuilder::default()
                    .policy_type(AutoBuildPolicyType::PERIODICAL)
                    .period_in_second(5000 as u64)
                    .build()?,
            )
            .build()?;
        let args = ModifyIndexArgsBuilder::default()
            .database(TESTDATABSE.to_string())
            .table(TESTTABLE.to_string())
            .index(index)
            .build()?;
        // let data = serde_json::to_string(&args)?;
        // println!("json data: {}", data);
        let ret = UTCLIENT.modify_index(&args).await?;
        println!("modify index: {:?}", ret);
        Ok(())
    }
}

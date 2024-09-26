use anyhow::Result;

/// before start index.rs example, you should reade the table.rs example first.
use mochow_sdk_rust::mochow::{
    api::{self, AutoBuildPolicyType, IndexType, MetricType},
    client,
};

const ACCOUNT: &str = "your_account";
const PASSWORD: &str = "your_password";
const ENDPOINT: &str = "http://127.0.0.1:5287";

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::MochowClient::new(ACCOUNT, PASSWORD, ENDPOINT).unwrap();

    // descript index 'vector_idx'
    let index_desc = client
        .desc_index("book2", "book_segments", "vector_idx")
        .await?;
    println!("{:?}", index_desc);

    // delete the vector index 'vector_idx' you created in table.rs example
    let _delete_ret_resp = client
        .delete_index("book2", "book_segments", "vector_idx")
        .await?;

    // recreate the index vector index 'vector_idx' you delete
    let indexes = vec![api::IndexSchemaBuilder::default()
        .index_name("vector_idx")
        .field("vector")
        .index_type(IndexType::HNSW)
        .metric_type(MetricType::L2)
        .params(api::VectorIndexParams::HNSW(api::HNSWIndexParam {
            m: 16,
            ef_construction: 200,
        }))
        .build()?];
    let create_index_args = api::CreateIndexArgsBuilder::default()
        .database("book2")
        .table("book_segments")
        .indexes(indexes)
        .build()?;
    let _create_index_resp = client.create_index(&create_index_args).await?;
    println!("create index: {:?}", _create_index_resp);

    // modify the vector index 'vector_idx' you created, only support the auto_build attribute now
    let index = api::IndexSchemaBuilder::default()
        .index_name("vector_idx")
        .auto_build(true)
        .auto_build_policy(
            api::AutoBuildPolicyBuilder::default()
                .policy_type(AutoBuildPolicyType::PERIODICAL)
                .period_in_second(5000 as u64)
                .build()?,
        )
        .build()?;
    let modify_index_args = api::ModifyIndexArgsBuilder::default()
        .database("book2")
        .table("book_segments")
        .index(index)
        .build()?;
    println!("modify index: {:?}", modify_index_args);
    Ok(())
}

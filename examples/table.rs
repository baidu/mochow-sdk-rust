use anyhow::Result;
// make sure you have read the database.rs example first, and created a database named 'book2'
// this example shows how to use the table api.
use mochow_sdk_rust::mochow::{
    api::{self, FieldType, IndexType, MetricType, PartitionType},
    client,
};

const ACCOUNT: &str = "your_account";
const PASSWORD: &str = "your_password";
const ENDPOINT: &str = "http://127.0.0.1:5287";

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::MochowClient::new(ACCOUNT, PASSWORD, ENDPOINT).unwrap();

    // table 'book_segments2' is created in the database 'book2'
    let _drop_table_resp = client.drop_table("book2", "book_segments2").await.unwrap();

    // create table 'book_segments'
    // field: id, bookName, author, page, vector,
    // index: book_name_idx, vector_idx
    let fields = vec![
        api::FieldSchemaBuilder::default()
            .field_name("id")
            .field_type(FieldType::STRING)
            .primary_key(true)
            .partition_key(true)
            .not_null(true)
            .build()?,
        api::FieldSchemaBuilder::default()
            .field_name("bookName")
            .field_type(FieldType::STRING)
            .not_null(true)
            .build()?,
        api::FieldSchemaBuilder::default()
            .field_name("author")
            .field_type(FieldType::STRING)
            .build()?,
        api::FieldSchemaBuilder::default()
            .field_name("page")
            .field_type(FieldType::UINT32)
            .build()?,
        api::FieldSchemaBuilder::default()
            .field_name("vector")
            .field_type(FieldType::FLOAT_VECTOR)
            .not_null(true)
            .dimension(3)
            .build()?,
    ];
    let indexes = vec![
        api::IndexSchemaBuilder::default()
            .index_name("book_name_idx")
            .field("bookName")
            .index_type(IndexType::HNSW)
            .build()?,
        api::IndexSchemaBuilder::default()
            .index_name("vector_idx")
            .field("vector")
            .index_type(IndexType::HNSW)
            .metric_type(MetricType::L2)
            .params(api::VectorIndexParams::HNSW(api::HNSWIndexParam {
                m: 32,
                ef_construction: 200,
            }))
            .build()?,
    ];
    let create_tabke_args = api::CreateTableArgsBuilder::default()
        .database("book2")
        .table("book_segments")
        .description("basic test")
        .replication(3 as u32)
        .partition(api::Partition {
            partition_type: PartitionType::HASH,
            partition_num: 3,
        })
        .schema(api::TableSchema {
            fields: fields,
            indexes: indexes,
        })
        .build()?;
    let create_table_resp = client.create_table(&create_tabke_args).await?;
    println!("create table resp: {:?}", create_table_resp);

    // get table info
    let description_of_table = client.desc_table("book2", "book_segments").await?;
    println!(
        "description of table fields: {:?}, filed count: {}",
        description_of_table.table.schema.fields,
        description_of_table.table.schema.fields.len()
    );
    println!(
        "description of table index: {:?}",
        description_of_table.table.schema.indexes
    );

    // you can add a field to table book_segments
    let add_fields = vec![api::FieldSchemaBuilder::default()
        .field_name("bookAlias")
        .field_type(FieldType::STRING)
        .build()?];
    let add_field_args = api::AddFieldArgsBuilder::default()
        .database("book2")
        .table("book_segments")
        .schema(api::TableSchema {
            fields: add_fields,
            indexes: vec![],
        })
        .build()?;
    let _add_field_resp = client.add_field(&add_field_args).await?;

    // get table info after add a field
    let description_of_table = client.desc_table("book2", "book_segments").await?;
    println!(
        "description of table fields: {:?}, filed count: {}",
        description_of_table.table.schema.fields,
        description_of_table.table.schema.fields.len()
    );
    println!(
        "description of table index: {:?}",
        description_of_table.table.schema.indexes
    );
    Ok(())
}

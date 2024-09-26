use anyhow::Result;
use serde::{Deserialize, Serialize};

const ACCOUNT: &str = "your_account";
const PASSWORD: &str = "your_password";
const ENDPOINT: &str = "http://127.0.0.1:5287";
const TESTDATABSE: &str = "book2";
const TESTTABLE: &str = "book_segments";

use mochow_sdk_rust::mochow::{api, client};

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

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::MochowClient::new(ACCOUNT, PASSWORD, ENDPOINT).unwrap();

    // insert row with a defined struct
    let insert_args1 = api::InsertRowArgsBuilder::default()
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
    let _ret = client.insert_row(&insert_args1).await?;
    println!("insert_row ret: {:?}", _ret);

    // insert row with a json object
    let insert_args2 = api::InsertRowArgsBuilder::default()
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
    let _ret = client.insert_row(&insert_args2).await?;
    println!("insert_row ret: {:?}", _ret);

    // query the data you insert
    let query_args = api::QueryRowArgsBuilder::default()
        .database(&TESTDATABSE.to_string())
        .table(&TESTTABLE.to_string())
        .primary_key(serde_json::json!({
            "id": "0001",
        }))
        .projections(vec!["id".to_string(), "bookName".to_string()])
        .retrieve_vector(false)
        .build()?;
    let query_ret = client.query_row::<MyRecord>(&query_args).await?;
    println!("query_row ret: {:?}", query_ret.row);
    let row1 = query_ret.row;
    let query_ret = client.query_row::<serde_json::Value>(&query_args).await?;
    println!("query_row ret: {:?}", query_ret.row);
    // convert json value to struct
    let row2 = serde_json::from_value(query_ret.row)?;
    assert_eq!(row1, row2);

    // update row
    let update_args = api::UpdateRowArgsBuilder::default()
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
    let _ret = client.update_row(&update_args).await?;
    println!("update_row ret: {:?}", _ret);

    // delete data
    // you can delete data by primary key or filter
    let delete_args = api::DeleteRowArgsBuilder::default()
        .database(&TESTDATABSE.to_string())
        .table(&TESTTABLE.to_string())
        // .primary_key(serde_json::json!({
        //     "id": "0001",
        // }))
        .filter("page >= 22")
        .build()?;
    let _ret = client.delete_rows(&delete_args).await?;
    println!("delete_row ret: {:?}", _ret);

    // upsert data
    let upsert_args = api::UpsertRowArgsBuilder::default()
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
    let _ret = client.upsert_row(&upsert_args).await?;
    println!("upsert_row ret: {:?}", _ret);

    // select data
    let mut select_args = api::SelectRowsArgsBuilder::default()
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
        let ret = client
            .select_rows::<serde_json::Value>(&select_args)
            .await?;
        println!("select_rows ret: {:?}", ret);
        if !ret.is_truncated {
            break;
        } else {
            select_args.marker = Some(ret.next_marker);
        }
    }

    // search data
    let search_args = api::SearchRowsArgsBuilder::default()
        .database(&TESTDATABSE.to_string())
        .table(&TESTTABLE.to_string())
        .anns(
            api::AnnsSearchParamsBuilder::default()
                .vector_field("vector")
                .vector_floats(vec![0.3123, 0.43, 0.213])
                .params(api::VectorSearchParams::HNSW(api::HNSWSearchParams {
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
    let ret = client
        .search_rows::<serde_json::Value>(&search_args)
        .await?;
    println!("search_rows ret: {:?}", ret.rows);

    // batch search data
    let batch_ann_params = api::BatchAnnsSearchParamsBuilder::default()
        .vector_field("vector")
        .vector_floats(vec![vec![0.3123, 0.43, 0.213], vec![0.5512, 0.33, 0.43]])
        .params(api::VectorSearchParams::HNSW(api::HNSWSearchParams {
            ef: 200,
            limit: 10,
            distance_far: None,
            distance_near: None,
            pruning: false,
        }))
        .filter("bookName = '三国演义'")
        .build()?;
    let batch_search_args = api::BatchSearchRowsArgsBuilder::default()
        .database(&TESTDATABSE.to_string())
        .table(&TESTTABLE.to_string())
        .anns(batch_ann_params)
        .retrieve_vector(true)
        .build()?;
    let batch_rets = client
        .batch_search_rows::<serde_json::Value>(&batch_search_args)
        .await?;
    for (i, bs) in batch_rets.results.iter().enumerate() {
        println!("batch: {}, {:?}", i, bs.search_vector_floats);
        for (j, ss) in bs.rows.iter().enumerate() {
            println!("{}, {:?}", j, ss);
        }
    }
    Ok(())
}

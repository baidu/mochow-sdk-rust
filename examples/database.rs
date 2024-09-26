use mochow_sdk_rust::{error, mochow};

const ACCOUNT: &str = "your_account";
const PASSWORD: &str = "your_password";
const ENDPOINT: &str = "http://127.0.0.1:5287";

#[tokio::main]
async fn main() {
    let client = mochow::client::MochowClient::new(ACCOUNT, PASSWORD, ENDPOINT).unwrap();
    // let _create_database_resp = client.create_database("book").await.unwrap();
    let _create_database_resp = client.create_database("book2").await;
    match _create_database_resp {
        Ok(ret) => println!("create database resp: {}", ret.msg),
        Err(e) => match e {
            error::SdkError::ServiceError(e) => {
                println!(
                    "create database ServiceError: {}, {:?}",
                    e.status_code, e.resp
                );
                assert_eq!(e.resp.code, 51);
                assert_eq!(e.resp.msg, "Database Already Exists".to_string());
                return;
            }
            _ => {
                println!("create database error: {}", e);
                return;
            }
        },
    }

    let list_database_resp = client.list_database().await.unwrap();

    // database list: ["book", "book2"]
    println!("database list: {:?}", list_database_resp.databases);

    // if you create database which already exists, it will return error
    // status_code: 400, request_id: xxxx, msg: {"code":51,"msg":"Database Already Exists"}
    // let _create_database_resp = client.create_database("book2").await.unwrap();

    // check table list at database "book2"
    // table list of database book2: ["book_segments2"]
    let table_list = client.list_table("book2").await.unwrap();
    println!("table list of database book2: {:?}", table_list.tables);

    // you can't drop database before you drop all tables in it
    //  status_code: 400, request_id: xxxx, msg: {"code":53,"msg":"Database Not Empty"}
    let _delete_database_resp = client.drop_database("book2").await.unwrap();
    println!("delete database resp: {}", _delete_database_resp.msg);
}

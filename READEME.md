# 百度向量数据库 Mochow RUST SDK

针对百度智能云向量数据库，我们推出了一套 RUST SDK（下称Mochow SDK），方便用户通过代码调用百度向量数据库。访问[openAPI](https://cloud.baidu.com/doc/VDB/s/als74z3nv) 了解更多信息。

## 如何安装
```shell
cargo install mochow-sdk-rust
```
也可以通过引入git仓库的方式
```toml
mochow-sdk-rust = { git = "https://github.com/baidu/mochow-sdk-rust" }
```

## 快速使用
在使用Mochow SDK 之前，用户需要在百度智能云上创建向量数据库，以获得 API Key。API Key 是用户在调用Mochow SDK 时所需要的凭证。具体获取流程参见平台的[向量数据库使用说明文档](https://cloud.baidu.com/)。

获取到 API Key 后，用户还需要传递它们来初始化Mochow SDK。 可以通过如下方式初始化Mochow SDK：

```rust
use mochow-sdk-rust::mochow;
const ACCOUNT: &str = "your_account";
const PASSWORD: &str = "your_password";
const ENDPOINT: &str = "http://127.0.0.1:5287";

#[tokio::main]
async fn main() {
    /// 初始化客户端
    let client = mochow::client::MochowClient::new(ACCOUNT, PASSWORD, ENDPOINT).unwrap();

    /// 也可以通过config初始化客户端
    let config = mochow::config::ClientConfigurationBuilder::default()
        .account(ACCOUNT)
        .api_key(PASSWORD)
        .endpoint(ENDPOINT)
        .build()
        .unwrap();
    let client = mochow::client::MochowClient::new_with_configuration(&config).unwrap();

    let _ = client.create_database("book").await.unwrap();
}
```



目前Mochow SDK 支持用户使用如下功能:

+ Databse 操作
+ Table 操作
+ Index 操作
+ Row 操作

## License

Apache-2.0
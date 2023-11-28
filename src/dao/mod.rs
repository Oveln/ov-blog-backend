pub mod article;
pub mod user;
use mongodb::{Client, Database};
use tokio::sync::OnceCell;

use crate::config::CONFIG;

const CLIENT_: OnceCell<Client> = OnceCell::const_new();

// 获取客户端实例
async fn get_client() -> Client {
    CLIENT_
        .get_or_init(|| async {
            Client::with_uri_str(&CONFIG.db_uri)
                .await
                .expect("Server Database Connect Error")
        })
        .await
        .clone()
}

// 获取数据库实例
async fn get_db() -> Database {
    get_client().await.database(&CONFIG.db_name)
}

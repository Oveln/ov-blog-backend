use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection, Cursor,
};

use crate::{
    data::user::{InsertUser, User, UserList},
    error::OvResult,
};

use super::get_db;

//数据库user存储User类型的数据
const COLLECTION_NAME: &str = "user";

// 获取user集合
async fn get_coll() -> Collection<Document> {
    get_db().await.collection(COLLECTION_NAME)
}

// 获取用户列表
pub async fn get_user_list_dao() -> OvResult<UserList> {
    let cursor: Cursor<Document> = get_coll().await.find(None, None).await?;
    cursor.try_collect::<Vec<_>>().await?.try_into()
}

// 通过id获取用户
pub async fn get_user_by_id_dao(user_id: &String) -> OvResult<Option<User>> {
    let user = get_coll()
        .await
        .find_one(doc! {"_id": user_id}, None)
        .await?;
    user.map(|user| User::try_from(user)).transpose()
}

// 通过用户名获取用户
pub async fn get_user_by_name_dao(user_name: &String) -> OvResult<Option<User>> {
    let user = get_coll()
        .await
        .find_one(doc! {"name": user_name}, None)
        .await?;
    user.map(|user| User::try_from(user)).transpose()
}

// 添加用户
pub async fn add_user_dao(user: InsertUser) -> OvResult<ObjectId> {
    let user: Document = user.into();
    let id = get_coll()
        .await
        .insert_one(user, None)
        .await?
        .inserted_id
        .as_object_id()
        .unwrap();
    Ok(id)
}

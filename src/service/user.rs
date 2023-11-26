use actix_web::web::Json;
use mongodb::bson::oid::ObjectId;

use crate::{
    dao::user::{add_user_dao, get_user_by_name_dao, get_user_list_dao},
    data::user::{InsertUser, User, UserList},
    error::OvError,
    restful_result::RestfulResult,
};

//获取用户列表
pub async fn get_user_list_service() -> RestfulResult<UserList> {
    get_user_list_dao().await.into()
}

//添加用户
pub async fn add_user_service(insert_user: Json<InsertUser>) -> RestfulResult<ObjectId> {
    let insert_user = insert_user.into_inner();
    match get_user_by_name_dao(&insert_user.name).await {
        Ok(user_option) => {
            match user_option {
                //如果用户已经存在，返回错误
                Some(_) => Err(OvError::UserAleadyExist),
                //如果用户不存在，添加用户
                None => add_user_dao(insert_user).await.into(),
            }
        }
        //如果数据库查询出错，返回错误
        Err(err) => Err(err),
    }
    .into()
}

// 通过用户名获取用户
pub async fn get_user_by_name_service(user_name: String) -> RestfulResult<Option<User>> {
    get_user_by_name_dao(&user_name).await.into()
}

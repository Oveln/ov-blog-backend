use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, Path},
    Responder,
};

use crate::{
    data::user::InsertUser,
    service::user::{add_user_service, get_user_by_name_service, get_user_list_service},
};

// 获取用户列表
#[get("/")]
async fn get_user_list() -> impl Responder {
    get_user_list_service().await
}

// 添加用户
#[post("/")]
async fn add_user(user: web::Json<InsertUser>) -> impl Responder {
    add_user_service(user).await
}

// 通过用户名获取用户
#[get("/{user_name}")]
async fn get_user(user_name: Path<String>) -> impl Responder {
    let user_name = user_name.into_inner();
    get_user_by_name_service(user_name).await
}

// 用户相关路由
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_user_list)
            .service(add_user)
            .service(get_user),
    );
}

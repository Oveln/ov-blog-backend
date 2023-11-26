use std::env::set_var;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::info;
use ov_blog_backend::{config::CONFIG, view::user::user_config};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello actix")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 设置日志级别
    set_var("RUST_LOG", "info");
    // 初始化日志
    env_logger::init();
    info!("start server");

    HttpServer::new(|| App::new().service(hello).configure(user_config))
        .bind((CONFIG.host.as_str(), CONFIG.port))
        .unwrap()
        .run()
        .await
}

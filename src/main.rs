use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::info;
use ov_blog_backend::{
    config::CONFIG,
    view::{article::article_config, user::user_config},
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello actix")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();
    info!("start server");

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hello)
            .configure(user_config)
            .configure(article_config)
    })
    .bind((CONFIG.host.as_str(), CONFIG.port))
    .unwrap()
    .run()
    .await
}

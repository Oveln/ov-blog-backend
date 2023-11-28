use actix_web::{
    delete, get, post, put,
    web::{self, Json, Path},
    Responder,
};

use crate::{
    data::article::{InsertArticle, UpdateArticle},
    service::article::{
        add_article_service, delete_article_service, get_article_by_id_service,
        get_article_list_service, update_article_service,
    },
};

#[get("/")]
async fn get_article_list() -> impl Responder {
    get_article_list_service(true).await
}

#[post("/")]
async fn add_article(insert_article: Json<InsertArticle>) -> impl Responder {
    add_article_service(insert_article).await
}

#[get("/{article_id}")]
async fn get_article(article_id: Path<String>) -> impl Responder {
    let article_id = article_id.into_inner();
    get_article_by_id_service(article_id).await
}

#[put("/{article_id}")]
async fn update_article(
    article_id: Path<String>,
    update_article: Json<UpdateArticle>,
) -> impl Responder {
    let article_id = article_id.into_inner();
    update_article_service(article_id, update_article).await
}

#[delete("/{article_id}")]
async fn delete_article(article_id: Path<String>) -> impl Responder {
    let article_id = article_id.into_inner();
    delete_article_service(article_id).await
}

pub fn article_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .service(get_article_list)
            .service(add_article)
            .service(get_article)
            .service(update_article)
            .service(delete_article),
    );
}

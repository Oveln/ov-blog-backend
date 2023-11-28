use std::str::FromStr;

use actix_web::web::Json;
use mongodb::bson::oid::ObjectId;

use crate::{
    dao::article::{
        add_article_dao, delete_article_dao, get_article_dao, get_article_list_dao,
        update_article_dao,
    },
    data::article::{
        Article, ArticleList, DeleteArticleResponse, InsertArticle, InsertArticleResponse,
        UpdateArticle, UpdateArticleResponse,
    },
    error::OvError,
    restful_result::RestfulResult,
};

pub async fn get_article_list_service(visible: bool) -> RestfulResult<ArticleList> {
    let list = get_article_list_dao(visible).await;
    list.into()
}

pub async fn get_article_by_id_service(article_id: String) -> RestfulResult<Option<Article>> {
    let article_id = ObjectId::from_str(&article_id);
    let article_id = match article_id {
        Ok(article_id) => article_id,
        Err(_) => return Err(OvError::InvalidObjectId).into(),
    };
    get_article_dao(&article_id).await.into()
}

pub async fn add_article_service(
    insert_article: Json<InsertArticle>,
) -> RestfulResult<InsertArticleResponse> {
    let insert_article = insert_article.into_inner();
    add_article_dao(insert_article).await.into()
}

pub async fn update_article_service(
    article_id: String,
    update_article: Json<UpdateArticle>,
) -> RestfulResult<UpdateArticleResponse> {
    let article_id = ObjectId::from_str(&article_id);
    let article_id = match article_id {
        Ok(article_id) => article_id,
        Err(_) => return Err(OvError::InvalidObjectId).into(),
    };
    let update_article = update_article.into_inner();
    update_article_dao(&article_id, update_article).await.into()
}

pub async fn delete_article_service(article_id: String) -> RestfulResult<DeleteArticleResponse> {
    let article_id = ObjectId::from_str(&article_id);
    let article_id = match article_id {
        Ok(article_id) => article_id,
        Err(_) => return Err(OvError::InvalidObjectId).into(),
    };
    delete_article_dao(&article_id).await.into()
}

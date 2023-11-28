use futures::TryStreamExt;
use log::debug;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};

use crate::{
    data::article::{
        Article, ArticleList, DeleteArticleResponse, InsertArticle, InsertArticleResponse,
        UpdateArticle, UpdateArticleResponse,
    },
    error::OvResult,
};

use super::get_db;

const COLLECTION_NAME: &str = "article";

pub async fn get_coll() -> Collection<Document> {
    get_db().await.collection(COLLECTION_NAME)
}

pub async fn add_article_dao(article: InsertArticle) -> OvResult<InsertArticleResponse> {
    let article: Document = article.into();
    let id = get_coll()
        .await
        .insert_one(article, None)
        .await?
        .inserted_id
        .as_object_id()
        .unwrap();
    Ok(InsertArticleResponse { _id: id })
}

//获取文章列表
//visible:是否可见unvisible的文章
pub async fn get_article_list_dao(visible: bool) -> OvResult<ArticleList> {
    let mut query = doc! {
        // "content": doc! {
        //     "$slice": 100,
        // },
        // "$limit": 10,
        // "$sort": doc! {
        //     "create_time": -1,
        // },
    };

    if !visible {
        query.insert("visible", !visible);
    }
    let cursor = get_coll().await.find(query, None).await?;
    cursor.try_collect::<Vec<_>>().await?.try_into()
}

pub async fn get_article_dao(article_id: &ObjectId) -> OvResult<Option<Article>> {
    let article = get_coll()
        .await
        .find_one(doc! { "_id": article_id }, None)
        .await?;
    article
        .map(|article| Article::try_from(article))
        .transpose()
}

pub async fn update_article_dao(
    article_id: &ObjectId,
    update_article: UpdateArticle,
) -> OvResult<UpdateArticleResponse> {
    let update_article: Document = update_article.into();
    let update_article = doc! {
        "$set": update_article,
    };
    let result = get_coll()
        .await
        .update_one(doc! { "_id": article_id }, update_article, None)
        .await?;
    debug!("update_article_dao: {:?}", result);
    if result.modified_count > 0 {
        Ok(UpdateArticleResponse {
            // _id: id.as_object_id().unwrap(),
        })
    } else {
        Err(crate::error::OvError::NotFound)
    }
}

pub async fn delete_article_dao(article_id: &ObjectId) -> OvResult<DeleteArticleResponse> {
    let result = get_coll()
        .await
        .delete_one(doc! { "_id": article_id }, None)
        .await?;
    if result.deleted_count == 1 {
        Ok(DeleteArticleResponse {})
    } else {
        Err(crate::error::OvError::NotFound)
    }
}

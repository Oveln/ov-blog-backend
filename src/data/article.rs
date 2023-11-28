use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::error::OvError;

#[derive(Serialize, Deserialize)]
pub struct InsertArticle {
    pub title: String,
    pub content: String,
    pub visible: bool,
}

impl Into<Document> for InsertArticle {
    fn into(self) -> Document {
        doc! {
            "title": self.title,
            "content": self.content,
            "visible": self.visible,
            "create_time": DateTime::now(),
            "update_time": DateTime::now(),
        }
    }
}

pub struct InsertArticleResponse {
    pub _id: ObjectId,
}

impl Serialize for InsertArticleResponse {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("InsertArticleResponse", 1)?;
        state.serialize_field("_id", &self._id.to_hex())?;
        state.end()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub content: Option<String>,
    pub visible: Option<bool>,
}

impl Into<Document> for UpdateArticle {
    fn into(self) -> Document {
        let mut doc = Document::new();
        if let Some(title) = self.title {
            doc.insert("title", title);
        }
        if let Some(content) = self.content {
            doc.insert("content", content);
        }
        if let Some(visible) = self.visible {
            doc.insert("visible", visible);
        }
        doc.insert("update_time", DateTime::now());
        doc
    }
}

//自动实现Serialize
#[derive(Serialize, Deserialize)]
pub struct UpdateArticleResponse {
    // 本来以为数据库返回结果里有id，结果没有，所以注释
    // pub _id: ObjectId,
}

// impl Serialize for UpdateArticleResponse {
//     fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
//         S: Serializer {
//         let mut state = serializer.serialize_struct("UpdateArticleResponse", 1)?;
//         state.serialize_field("_id", &self._id.to_hex())?;
//         state.end()
//     }

// }

#[derive(Serialize, Deserialize)]
pub struct DeleteArticleResponse {}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub id: ObjectId,
    pub title: String,
    pub content: String,
    pub visible: bool,
    pub create_name: DateTime,
    pub update_name: DateTime,
}

impl TryFrom<Document> for Article {
    type Error = OvError;

    fn try_from(value: Document) -> Result<Self, Self::Error> {
        let id = value.get_object_id("_id")?;
        let title = value.get_str("title")?.to_string();
        let content = value.get_str("content")?.to_string();
        let visible = value.get_bool("visible")?;
        let create_name = value.get_datetime("create_time")?.clone();
        let update_name = value.get_datetime("update_time")?.clone();
        Ok(Article {
            id,
            title,
            content,
            visible,
            create_name,
            update_name,
        })
    }
}

impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Article", 6)?;
        state.serialize_field("id", &self.id.to_hex())?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("visible", &self.visible)?;
        state.serialize_field("create_time", &self.create_name.to_string())?;
        state.serialize_field("update_time", &self.update_name.to_string())?;
        state.end()
    }
}

#[derive(Debug, Serialize)]
pub struct ArticleList {
    pub list: Vec<Article>,
}

impl TryFrom<Vec<Document>> for ArticleList {
    type Error = OvError;

    fn try_from(value: Vec<Document>) -> Result<Self, Self::Error> {
        let list = value
            .into_iter()
            .map(|doc| Article::try_from(doc))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ArticleList { list })
    }
}

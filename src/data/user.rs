use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use crate::error::OvError;

#[derive(Serialize, Deserialize)]
pub struct InsertUser {
    pub name: String,
    pub password: String,
}

impl Into<Document> for InsertUser {
    fn into(self) -> Document {
        doc! {
            "name": self.name,
            "password": self.password,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl TryFrom<Document> for User {
    type Error = OvError;

    fn try_from(value: Document) -> Result<Self, Self::Error> {
        let id = value.get_object_id("_id").unwrap().to_hex();
        let name = value.get_str("name").unwrap().to_string();
        let password = value.get_str("password").unwrap().to_string();
        Ok(User { id, name, password })
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserList {
    #[serde(rename = "user_list")]
    pub user_list: Vec<User>,
}

impl TryFrom<Vec<Document>> for UserList {
    type Error = OvError;

    fn try_from(value: Vec<Document>) -> Result<Self, Self::Error> {
        Ok(UserList {
            user_list: value
                .into_iter()
                .map(|user| User::try_from(user))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

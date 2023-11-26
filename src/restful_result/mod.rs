use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::error::OvResult;

#[derive(Serialize, Deserialize)]
//RESTful API 返回结果
pub struct RestfulResult<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> Into<HttpResponse> for RestfulResult<T> {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl<T> From<OvResult<T>> for RestfulResult<T>
where
    T: Serialize,
{
    fn from(result: OvResult<T>) -> Self {
        match result {
            Ok(data) => RestfulResult {
                code: 0,
                msg: "success".to_string(),
                data: Some(data),
            },
            Err(err) => RestfulResult {
                code: -1,
                msg: err.to_string(),
                data: None,
            },
        }
    }
}

impl<T> Responder for RestfulResult<T>
where
    T: Serialize,
{
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }

    type Body = BoxBody;
}

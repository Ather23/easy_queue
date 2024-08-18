use axum::body::Body;
use axum::http::{ Response, StatusCode };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize)]
pub struct ResponseHandler<T> where T: Serialize {
    pub payload: T,
}

impl<T: Serialize> ResponseHandler<T> {
    pub fn new(payload: T) -> Self {
        Self { payload }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|err| {
            let err = ApiErrorResponse {
                msg: "Serialization error".to_string(),
            };
            serde_json::to_string(&err).unwrap().to_string()
        })
    }

    pub fn response(&self, status: &StatusCode) -> axum::http::Response<Body> {
        Response::builder().status(status).body(Body::from(self.to_json())).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiErrorResponse {
    pub msg: String,
}

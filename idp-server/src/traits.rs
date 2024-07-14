use async_trait::async_trait;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait JwksHandler {
    async fn jwks(&self) -> impl Responder;
}

#[async_trait]
pub trait AuthHandler {
    async fn authenticate(&self, credentials: web::Json<Credentials>) -> HttpResponse;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

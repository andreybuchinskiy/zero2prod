use actix_web::web;
use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

pub async fn publish_newsletters(_body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

use actix_web::{body::BoxBody, get, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[get("/rust/wherearewe")]
async fn wherearewe() -> impl Responder {
    HttpResponse::Ok().body("Universe")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .workers(2)
        .bind(("127.0.0.1", 8900))?
        .run()
        .await
}

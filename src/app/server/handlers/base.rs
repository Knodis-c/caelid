use actix_web::{
    HttpResponse,
    HttpRequest,
    http::header::ContentType,
    web,
    Result,
};
use crate::app::server::shared::template::{Context, Engine};

pub async fn index(_req: HttpRequest, template_engine: web::Data<Engine>) -> Result<HttpResponse> {
    let body = template_engine.render("index", &Context::new())?;

    let res = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body);

    Ok(res)
}

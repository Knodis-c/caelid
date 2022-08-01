use actix_web::{
    HttpResponse,
    HttpRequest,
    http::header::ContentType,
    web,
    Result,
};
use crate::app::server::shared::template::{Context, Engine};

pub async fn index(_req: HttpRequest, template_engine: web::Data<Engine>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("component", &template_engine.react_component());
    ctx.insert("msg", "Hello World");

    let body = template_engine.render_v1("base/index", Some(ctx))?;

    let res = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body);

    Ok(res)
}

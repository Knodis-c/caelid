use actix_web::{
    HttpResponse,
    HttpRequest,
    http::header::ContentType,
    web,
    Result,
};
use crate::{
    app::template_engine::{Context, Engine},
    internal::json::JSON
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestProps {
    a: usize,
    b: usize
}

impl JSON<'_> for TestProps {}

pub async fn index(_req: HttpRequest, template_engine: web::Data<Engine>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    let props = TestProps { a: 3, b: 4 };
    let props_json = props.to_json()?;

    let react_component = template_engine.react_component("test/index", Some(&props_json));

    ctx.insert("component", &react_component);
    ctx.insert("msg", "Hello World");

    let body = template_engine.render_v1("base/index", Some(ctx))?;

    let res = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body);

    Ok(res)
}

use crate::app::database::pg::PgConnPool;
use crate::app::server::handlers::prelude::*;

pub async fn read(_req: HttpRequest, _pg: web::Data<PgConnPool>) -> Result<HttpResponse> {
    let res = HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Hello");

    Ok(res)
}

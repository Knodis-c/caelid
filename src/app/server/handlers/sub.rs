use actix_web::{HttpRequest, web};

pub async fn test(_req: HttpRequest, path: web::Path<String>) -> String {
    format!("{}", path)
}

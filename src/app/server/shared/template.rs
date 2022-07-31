use actix_web::{
    error::InternalError,
    http::StatusCode,
    web
};

const PATH_TO_HTML: &'static str = "assets/html/**/*.html";

pub struct Engine {
    engine: tera::Tera
}

pub type Context = tera::Context;

impl Engine {
    pub fn init() -> Result<web::Data<Self>, tera::Error> {
        let tera = tera::Tera::new(PATH_TO_HTML)?;
        let engine = web::Data::new(Engine { engine: tera });
        Ok(engine)
    }

    pub fn render(&self, name: &str, ctx: &tera::Context) -> actix_web::Result<String> {
        let name = format!("{name}.html");
        let html = self.engine
            .render(&name, ctx)
            .map_err(|e| InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(html)
    }
}

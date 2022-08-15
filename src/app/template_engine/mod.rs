use actix_web::{
    error::InternalError,
    http::StatusCode
};
use crate::lib::json::JSONStr;
use std::fs;
use std::net::{Ipv4Addr, SocketAddrV4};

/// Path to app's HTML files.
const PATH_TO_HTML: &'static str = "assets/html/**/*.html";

/// Templating engine wrapped around `Tera`.
pub struct Engine {
    engine: tera::Tera
}

pub type Context = tera::Context;
type EngineResult<T> = actix_web::Result<T>;

impl Engine {
    /// Initializes a shareable instance of `Engine` across all routes/handlers.
    pub fn init() -> Result<Self, tera::Error> {
        let tera = tera::Tera::new(PATH_TO_HTML)?;
        let engine = Engine { engine: tera };
        Ok(engine)
    }

    /// Loads processed HTML from provided `name` and `ctx` arguments. The `.html` extension must
    /// be ommitted from the provided `name` argument.
    pub fn render(&self, name: &str, ctx: &tera::Context) -> EngineResult<String> {
        let name = format!("{name}.html");
        let html = self.engine
            .render(&name, ctx)
            .map_err(|e| InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(html)
    }

    /// Loads processed HTML using a default context with optional additional contexts.
    pub fn render_v1(&self, name: &str, additional_ctx: Option<tera::Context>) -> EngineResult<String> {
        let mut ctx = Self::base_ctx_v1();

        if let Some(c) = additional_ctx {
            ctx.extend(c);
        }

        self.render(name, &ctx)
    }

    /// Generates an HTML string that is loaded into the template, which is to be the target
    /// container for the React component. The `component_name` argument uses
    /// `assets/javascript/components` as the root directory, so for the component,
    /// `assets/javascript/components/some_component/index`, the `component_name` parameter should
    /// be `some_component/index`.
    pub fn react_component(&self, component_name: &str, props: Option<&JSONStr>) -> String {
        match props {
            Some(p) => format!("<div data-react-class='{component_name}' data-react-props='{p}'></div>"),
            None => format!("<div data-react-class='{component_name}'></div>")
        }
    }

    /// Base context.
    fn base_ctx_v1() -> tera::Context {
        let mut ctx = tera::Context::new();

        ctx.insert("application_v1_js", &Self::application_v1_js());

        ctx
    }

    #[cfg(any(debug_assertions, test))]
    fn application_v1_js() -> String {
        use crate::app::server::assets::{PUBLIC_PATH, STATIC_ASSETS_PATH};

        let webpack_port = dotenv::var("WEBPACK_DEV_SERVER_PORT")
            .unwrap_or("8080".to_owned())
            .parse::<u16>()
            .unwrap();

        let host = Ipv4Addr::new(127, 0, 0, 1);
        let socket_addr = SocketAddrV4::new(host, webpack_port);

        let mut entries = fs::read_dir(STATIC_ASSETS_PATH).unwrap();
        let maybe_javascript = entries.find(|dir_entry| {
            if let Ok(entry) = dir_entry {
                let name = entry.file_name().into_string().unwrap();
                return name.contains("application_v1") && name.contains(".js")
            }

            false
        }).unwrap();

        let javascript = maybe_javascript
            .and_then(|entry| Ok(entry.file_name().into_string().unwrap()))
            .unwrap();

        format!(
            "http://{}{}/{}",
            socket_addr.to_string(),
            PUBLIC_PATH,
            javascript
        )
    }

    #[cfg(not(debug_assertions))]
    fn application_v1_js() -> String {
        todo!();
        "".to_string()
    }
}

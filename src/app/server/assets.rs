use actix_files as fs;
use actix_web::web;

pub const STATIC_ASSETS_PATH: &'static str = "./public";
pub const PUBLIC_PATH: &'static str = "/static";

pub fn static_assets(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new(PUBLIC_PATH, STATIC_ASSETS_PATH).show_files_listing());
}

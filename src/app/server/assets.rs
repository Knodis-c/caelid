use actix_files as fs;
use actix_web::web;

pub const STATIC_ASSETS_PATH: &'static str = "./assets";

//let file_path: PathBuf = [STATIC_ASSETS_PATH, "index.html"].iter().collect();
//Ok(fs::NamedFile::open(file_path)?)


pub fn static_assets(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new("/static", STATIC_ASSETS_PATH).show_files_listing());
}

pub fn init_logger() {
    log4rs::init_file("config/logger.yaml", Default::default())
        .expect("Failed to initialize logger");
}

use env_logger::Env;

pub fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "caelid=trace")
        .write_style_or("LOG_STYLE", "auto");

    env_logger::init_from_env(env);
}

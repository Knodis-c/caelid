pub mod graphics {
    pub fn bold(txt: &str) -> String {
        format!("\x1b[1m{}\x1b[0m", txt)
    }
}

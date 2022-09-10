#![allow(dead_code)]

pub mod colors {
    pub fn bold(txt: &str) -> String {
        format!("\x1b[1m{}\x1b[22m", txt)
    }
    
    pub fn dim(txt: &str) -> String {
        format!("\x1b[2m{}\x1b[22m", txt)
    }

    pub fn black_fg(txt: &str) -> String {
        format!("\x1b[30m{}\x1b[0m", txt)
    }

    pub fn red_fg(txt: &str) -> String {
        format!("\x1b[31m{}\x1b[0m", txt)
    }

    pub fn green_fg(txt: &str) -> String {
        format!("\x1b[32m{}\x1b[0m", txt)
    }

    pub fn yellow_fg(txt: &str) -> String {
        format!("\x1b[33m{}\x1b[0m", txt)
    }

    pub fn blue_fg(txt: &str) -> String {
        format!("\x1b[34m{}\x1b[0m", txt)
    }

    pub fn purple_fg(txt: &str) -> String {
        format!("\x1b[35m{}\x1b[0m", txt)
    }

    pub fn cyan_fg(txt: &str) -> String {
        format!("\x1b[36m{}\x1b[0m", txt)
    }

    pub fn white_fg(txt: &str) -> String {
        format!("\x1b[37m{}\x1b[0m", txt)
    }
}

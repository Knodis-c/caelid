use crate::internal::ansi::colors;
use tracing_core::Level;

pub fn ansi_log_level(level: &Level) -> String {
    if level == &Level::TRACE {
        colors::blue_fg(level.as_str())
    } else if level == &Level::DEBUG {
        colors::green_fg(level.as_str())
    } else if level == &Level::INFO {
        colors::cyan_fg(level.as_str())
    } else if level == &Level::WARN {
        colors::yellow_fg(level.as_str())
    } else if level == &Level::ERROR {
        colors::red_fg(level.as_str())
    } else {
        colors::white_fg(level.as_str())
    }
}

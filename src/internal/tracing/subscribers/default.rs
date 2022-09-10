use tracing_subscriber::fmt::{format::FmtSpan, Subscriber, time::UtcTime};
use super::super::formatters::default::DefaultFormatter;

/// Initializes the global default subscriber for tracing events.
pub fn init() {
    Subscriber::builder()
        .with_span_events(FmtSpan::NONE)
        .event_format(DefaultFormatter::new())
        .init()
}

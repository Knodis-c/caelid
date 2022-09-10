use crate::internal::ansi::colors;
use std::fmt;
use super::super::utils::ansi_log_level;
use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,
    time::{FormatTime, UtcTime},
};
use tracing_subscriber::registry::LookupSpan;

pub struct DefaultFormatter;

impl DefaultFormatter {
    pub fn new() -> Self {
        DefaultFormatter {}
    }
}

impl<S, N> FormatEvent<S, N> for DefaultFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let event_metadata = event.metadata();
        let level = event_metadata.level();
        let module = event_metadata.target();

        write!(&mut writer, "{}", colors::dim("["))?;

        UtcTime::rfc_3339().format_time(&mut writer)?;

        write!(
            &mut writer, " {} {}",
            ansi_log_level(&level),
            module,
        )?;


        if let Some(scope) = ctx.event_scope() {
            let mut iter = scope.from_root().peekable();

            while let Some(span) = iter.next() {
                write!(writer, " {}<{}>", span.name(), span.id().into_u64())?;

                let ext = span.extensions();

                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }

                if let Some(_) = iter.peek() {
                    write!(writer, " {}", colors::cyan_fg("\u{2192}"))?;
                }
            }
        }

        write!(&mut writer, "{}", colors::dim("] "))?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

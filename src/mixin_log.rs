use crate::Log;

impl log::Log for crate::Log {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        Log::is_enabled()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = format!("{}", record.args());
        match record.level() {
            log::Level::Error => crate::Log::e(msg),
            log::Level::Warn => crate::Log::w(msg),
            log::Level::Info => crate::Log::i(msg),
            log::Level::Debug => crate::Log::d(msg),
            log::Level::Trace => crate::Log::v(msg),
        };
    }

    fn flush(&self) {
        // no-op
    }
}

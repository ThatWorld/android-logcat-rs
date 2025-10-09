impl log::Log for crate::logcat::Log {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = format!("{}", record.args());
        match record.level() {
            log::Level::Error => crate::logcat::Log::e(msg),
            log::Level::Warn => crate::logcat::Log::w(msg),
            log::Level::Info => crate::logcat::Log::i(msg),
            log::Level::Debug => crate::logcat::Log::d(msg),
            log::Level::Trace => crate::logcat::Log::v(msg),
        };
    }

    fn flush(&self) {
        // no-op
    }
}

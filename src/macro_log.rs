/// Log an `info` message to Android logcat
#[macro_export]
macro_rules! i {
    // android_logcat::i!(tag: "my_tag", "a {} message.", "log");
    (tag: $tag:expr, $($arg:tt)+) => {
        $crate::Log::info($tag, &format!($($arg)*));
    };

    // android_logcat::i!("a {} message.", "log");
    ($($arg:tt)*) => {
        $crate::Log::i(&format!($($arg)*));
    };
}

/// Log a `debug` message to Android logcat
#[macro_export]
macro_rules! d {
    // android_logcat::d!(tag: "my_tag", "a {} message.", "log");
    (tag: $tag:expr, $($arg:tt)+) => {
        $crate::Log::debug($tag, &format!($($arg)*));
    };

    // android_logcat::d!("a {} message.", "log");
    ($($arg:tt)*) => {
        $crate::Log::d(&format!($($arg)*));
    };
}

/// Log a `error` message to Android logcat
#[macro_export]
macro_rules! e {
    // android_logcat::d!(tag: "my_tag", "a {} message.", "log");
    (tag: $tag:expr, $($arg:tt)+) => {
        $crate::Log::error($tag, &format!($($arg)*));
    };

    // android_logcat::d!("a {} message.", "log");
    ($($arg:tt)*) => {
        $crate::Log::e(&format!($($arg)*));
    };
}

/// Log a `warn` message to Android logcat
#[macro_export]
macro_rules! w {
    // android_logcat::w!(tag: "my_tag", "a {} message.", "log");
    (tag: $tag:expr, $($arg:tt)+) => {
        $crate::Log::warn($tag, &format!($($arg)*));
    };

    // android_logcat::w!("a {} message.", "log");
    ($($arg:tt)*) => {
        $crate::Log::w(&format!($($arg)*));
    };
}

/// Log a `warn` message to Android logcat
#[macro_export]
macro_rules! v {
    // android_logcat::v!(tag: "my_tag", "a {} message.", "log");
    (tag: $tag:expr, $($arg:tt)+) => {
        $crate::Log::verbose($tag, &format!($($arg)*));
    };

    // android_logcat::v!("a {} message.", "log");
    ($($arg:tt)*) => {
        $crate::Log::v(&format!($($arg)*));
    };
}

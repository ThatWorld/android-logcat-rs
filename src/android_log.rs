use std::{
    ffi::CString,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
};

use crate::sys::{LogPriority, __android_log_print, __android_log_write};

/// Default tag used when no tag is specified
static DEFAULT_TAG: &str = "RustLog";

/// Global static instance of Log tag
static GLOBAL_LOG_TAG: OnceLock<String> = OnceLock::new();

/// Global static instance to enable or disable logging
static GLOBAL_LOG_ENABLED: AtomicBool = AtomicBool::new(true);

/// A struct for Android logging
pub struct Log;

impl Log {
    /// Print formatted log message
    pub fn print(prio: LogPriority, tag: &str, msg: &str) {
        if !Self::is_enabled() {
            return;
        }

        unsafe {
            __android_log_print(
                prio as i32,
                CString::new(tag).unwrap().as_ptr(),
                CString::new("%s").unwrap().as_ptr(),
                CString::new(msg).unwrap().as_ptr(),
            );
        }
    }

    /// Write simple log message
    pub fn write<T>(prio: LogPriority, tag: &str, msg: &str) {
        if !Self::is_enabled() {
            return;
        }

        unsafe {
            __android_log_write(
                prio as i32,
                CString::new(tag).unwrap().as_ptr(),
                CString::new(msg).unwrap().as_ptr(),
            );
        }
    }

    /// Log verbose message
    /// # Arguments
    /// * `tag` - The tag of the log message
    /// * `msg` - The log message
    pub fn verbose<T>(tag: T, msg: T)
    where
        T: AsRef<str>,
    {
        Self::print(LogPriority::VERBOSE, tag.as_ref(), msg.as_ref());
    }

    /// Log debug message
    /// # Arguments
    /// * `tag` - The tag of the log message
    /// * `msg` - The log message
    pub fn debug<T>(tag: T, msg: T)
    where
        T: AsRef<str>,
    {
        Self::print(LogPriority::DEBUG, tag.as_ref(), msg.as_ref());
    }

    /// Log debug message
    /// # Arguments
    /// * `tag` - The tag of the log message
    /// * `msg` - The log message
    pub fn info<T>(tag: T, msg: T)
    where
        T: AsRef<str>,
    {
        Self::print(LogPriority::INFO, tag.as_ref(), msg.as_ref());
    }

    /// Log warn message
    /// # Arguments
    /// * `tag` - The tag of the log message
    /// * `msg` - The log message
    pub fn warn<T>(tag: T, msg: T)
    where
        T: AsRef<str>,
    {
        Self::print(LogPriority::WARN, tag.as_ref(), msg.as_ref());
    }

    /// Log error message
    /// # Arguments
    /// * `tag` - The tag of the log message
    /// * `msg` - The log message
    pub fn error<T>(tag: T, msg: T)
    where
        T: AsRef<str>,
    {
        Self::print(LogPriority::ERROR, tag.as_ref(), msg.as_ref());
    }

    /// Log verbose message using the global tag
    /// If the global Log instance is not initialized, use the default tag [DEFAULT_TAG]
    /// # Arguments
    /// * `msg` - The log message
    pub fn v<T>(msg: T)
    where
        T: AsRef<str>,
    {
        if let Some(tag) = GLOBAL_LOG_TAG.get() {
            Self::verbose(tag.as_ref(), msg.as_ref());
        } else {
            Self::verbose(DEFAULT_TAG, msg.as_ref());
        }
    }

    /// Log debug message using the global tag
    /// If the global Log instance is not initialized, use the default tag [DEFAULT_TAG]
    /// # Arguments
    /// * `msg` - The log message
    pub fn d<T>(msg: T)
    where
        T: AsRef<str>,
    {
        if let Some(tag) = GLOBAL_LOG_TAG.get() {
            Self::debug(tag.as_ref(), msg.as_ref());
        } else {
            Self::debug(DEFAULT_TAG, msg.as_ref());
        }
    }

    /// Log info message using the global tag
    /// If the global Log instance is not initialized, use the default tag [DEFAULT_TAG]
    /// # Arguments
    /// * `msg` - The log message
    pub fn i<T>(msg: T)
    where
        T: AsRef<str>,
    {
        if let Some(tag) = GLOBAL_LOG_TAG.get() {
            Self::info(tag.as_ref(), msg.as_ref());
        } else {
            Self::info(DEFAULT_TAG, msg.as_ref());
        }
    }

    /// Log warn message using the global tag
    /// If the global Log instance is not initialized, use the default tag [DEFAULT_TAG]
    /// # Arguments
    /// * `msg` - The log message
    pub fn w<T>(msg: T)
    where
        T: AsRef<str>,
    {
        if let Some(tag) = GLOBAL_LOG_TAG.get() {
            Self::warn(tag.as_ref(), msg.as_ref());
        } else {
            Self::warn(DEFAULT_TAG, msg.as_ref());
        }
    }

    /// Log error message using the global tag
    /// If the global Log instance is not initialized, use the default tag [DEFAULT_TAG]
    /// # Arguments
    /// * `msg` - The log message
    pub fn e<T>(msg: T)
    where
        T: AsRef<str>,
    {
        if let Some(tag) = GLOBAL_LOG_TAG.get() {
            Self::error(tag.as_ref(), msg.as_ref());
        } else {
            Self::error(DEFAULT_TAG, msg.as_ref());
        }
    }

    /// Initialize the global Log instance with a specific tag
    /// # Arguments
    /// * `tag` - The tag to be used for the global Log instance
    /// * `mixinlog` - If true, set the Rust log crate to use this Log instance as the logger. e.g. log::info!("message")
    pub fn init(tag: &str, mixinlog: bool) {
        let _tag = GLOBAL_LOG_TAG.get_or_init(|| tag.to_string());
        let _enable = GLOBAL_LOG_ENABLED.store(true, Ordering::Relaxed);

        if mixinlog {
            log::set_max_level(log::LevelFilter::max());
            log::set_logger(&Self).unwrap();
        }
    }

    /// Get the tag of the Log instance
    pub fn tag() -> &'static str {
        if let Some(tag) = GLOBAL_LOG_TAG.get() {
            tag.as_ref()
        } else {
            DEFAULT_TAG
        }
    }

    /// Enable or disable logging
    /// # Arguments
    /// * `enabled` - If true, enable logging; if false, disable logging
    pub fn enabled(enabled: bool) {
        GLOBAL_LOG_ENABLED.store(enabled, Ordering::Relaxed);
    }

    /// Check if logging is enabled
    pub fn is_enabled() -> bool {
        GLOBAL_LOG_ENABLED.load(Ordering::Relaxed)
    }
}

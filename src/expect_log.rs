use std::fmt::Debug;

use crate::logcat::Log;

pub trait ExpectLogcat<T> {
    /// Expect with Log.e
    /// If the Result is Err, log the error message using Log.e and panic with the same message.
    /// If the Result is Ok, return the value.
    fn expect_log(self, msg: &str) -> T;
}

/// Implement ExpectLogcat for Result<T, E> where E: Debug
impl<T, E: Debug> ExpectLogcat<T> for Result<T, E> {
    fn expect_log(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                Log::e(&format!("{}: {:?}", msg, e));
                panic!("{}: {:?}", msg, e);
            }
        }
    }
}

/// Implement ExpectLogcat for Option<T> to handle None case
impl<T> ExpectLogcat<T> for Option<T> {
    fn expect_log(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                Log::e(msg);
                panic!("{}", msg);
            }
        }
    }
}

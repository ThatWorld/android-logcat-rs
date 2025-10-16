use std::fmt::Debug;

use crate::android_log::Log;

pub trait ExpectLogcat<T> {
    /// Expect with Log.e
    /// If the Result is Err, log the error message using Log.e and panic with the same message.
    /// If the Result is Ok, return the value.
    fn expect_log<S>(self, msg: S) -> T
    where
        S: AsRef<str>;
}

/// Implement ExpectLogcat for Result<T, E> where E: Debug
impl<T, E: Debug> ExpectLogcat<T> for Result<T, E> {
    fn expect_log<S>(self, msg: S) -> T
    where
        S: AsRef<str>,
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                Log::e(&format!("{}: {:?}", msg.as_ref(), e));
                panic!("{}: {:?}", msg.as_ref(), e);
            }
        }
    }
}

/// Implement ExpectLogcat for Option<T> to handle None case
impl<T> ExpectLogcat<T> for Option<T> {
    fn expect_log<S>(self, msg: S) -> T
    where
        S: AsRef<str>,
    {
        match self {
            Some(v) => v,
            None => {
                Log::e(msg.as_ref());
                panic!("{}", msg.as_ref());
            }
        }
    }
}

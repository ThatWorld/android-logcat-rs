# android-logcat

[![crates.io](https://img.shields.io/crates/v/android-logcat.svg)](https://crates.io/crates/android-logcat)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/thatworld/android-logcat-rs/blob/main/LICENSE.txt)

A simple Rust library for Android logcat. This library provides a simple interface to the Android logging system.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
android-logcat = "0.1"
log = "0.4"
```

### Basic Usage

You can use the `Log` struct to log messages with different priorities.

```rust
use android_logcat::Log;

Log::info("MyTag", "This is an info message");
Log::error("MyTag", "This is an error message");
```

### Global Logger

For convenience, you can initialize a global logger with a default tag.

```rust
use android_logcat::Log;

// In your app's initialization code
Log::init("MyAppTag", false);

// Later in your code
Log::i("Info message using the global tag");
Log::e("Error message using the global tag");
```

### With `log` crate

This library can be used as a backend for the `log` crate.

```rust
use android_logcat::Log;
use log::{info, error};

// In your app's initialization code
// Set the second parameter to true to mix in with the log crate
Log::init("MyAppTag", true);

// Now you can use the macros from the log crate
info!("This will be logged with MyAppTag");
error!("This is another error message");
```

### `expect_log` for `Result` and `Option`

The `ExpectLogcat` trait provides a convenient way to handle `Result` and `Option` types. If the value is `Err` or `None`, it logs an error message and then panics.

```rust
use android_logcat::ExpectLogcat;

fn might_fail() -> Result<(), &'static str> {
    Err("something went wrong")
}

fn run() {
    might_fail().expect_log("Failed to run");
}
```

This will log an error with the message "Failed to run: something went wrong" and then panic.

It works similarly for `Option`:

```rust
use android_logcat::ExpectLogcat;

let an_option: Option<i32> = None;
let value = an_option.expect_log("The option was None!");
```

This will log an error with the message "The option was None!" and then panic.

## License

This project is licensed under the Apache-2.0 license. See the [LICENSE.txt](LICENSE.txt) file for details.

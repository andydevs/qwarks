//! Logging to the browser console.

/// Logs a formatted message with `console.log`.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        ::web_sys::console::log_1(&::std::format!($($arg)*).into())
    };
}

/// Logs a formatted message with `console.warn`.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        ::web_sys::console::warn_1(&::std::format!($($arg)*).into())
    };
}

/// Logs a formatted message with `console.error`.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ::web_sys::console::error_1(&::std::format!($($arg)*).into())
    };
}

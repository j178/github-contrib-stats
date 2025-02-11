// https://github.com/rustwasm/console_error_panic_hook#readme
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_once as set_panic_hook;

#[cfg(not(feature = "console_error_panic_hook"))]
#[inline]
pub fn set_panic_hook() {}

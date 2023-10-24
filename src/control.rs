//! Control whether or not to colorize output.

use core::sync::atomic::AtomicBool;
use std::{env, io::IsTerminal};

lazy_static::lazy_static! {
    /// Whether or not to colorize output (Global).
    pub(crate) static ref SHOULD_COLORIZE: AtomicBool = {
        (std::io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none()).into()
    };
}

/// Whether or not to colorize output (Global).
#[inline(always)]
pub fn should_colorize() -> bool {
    SHOULD_COLORIZE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Set whether or not to colorize output (Global).
pub fn set_should_colorize(should: bool) {
    SHOULD_COLORIZE.store(should, std::sync::atomic::Ordering::Relaxed)
}

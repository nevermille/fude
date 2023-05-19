/// Logs an info if log feature is enabled
macro_rules! info {
    ($($x:tt)*) => (
        #[cfg(feature = "log")] {
            log::info!($($x)*)
        }
    )
}

/// Logs an error if log feature is enabled
macro_rules! error {
    ($($x:tt)*) => (
        #[cfg(feature = "log")] {
            log::error!($($x)*)
        }
    )
}

/// Logs a warning if log feature is enabled
macro_rules! warning {
    ($($x:tt)*) => (
        #[cfg(feature = "log")] {
            log::warn!($($x)*)
        }
    )
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use warning;

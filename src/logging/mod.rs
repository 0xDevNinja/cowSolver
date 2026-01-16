//! Logging utilities for the CowSolver project.
//!
//! This module provides initialization for the logger used by the
//! application. It uses the `env_logger` crate to configure log output
//! based on the `RUST_LOG` environment variable. If `RUST_LOG` is not
//! set, a default log level of `info` is used.

use env_logger::Env;

/// Initialize the global logger.
///
/// This function should be called at the start of CLI binaries and daemon
/// services to ensure that log messages are properly formatted and emitted.
/// The logger will read the `RUST_LOG` environment variable to set the
/// maximum log level. If the variable is not present, it defaults to `info`.
pub fn init_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
}

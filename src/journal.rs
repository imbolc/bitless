//! Journald logging initialization
use displaydoc::Display;
use std::io;
use thiserror::Error;
use tracing_subscriber::{filter::FromEnvError, fmt, prelude::*, registry, EnvFilter};

/// Logging initialization error
#[derive(Debug, Display, Error)]
pub enum Error {
    /// can't parse `RUST_LOG` env var
    Env(#[from] FromEnvError),
    /// can't connect to journald
    Journal(#[from] io::Error),
}

/// Initialises logging with formating based on `RUST_LOG` environment variable
/// Writes logs to journal with `to_journal` enabled or to `stdout` otherwise
pub fn init(to_journal: bool) -> Result<(), Error> {
    let filter = EnvFilter::try_from_default_env()?;
    if to_journal {
        let journal_layer = tracing_journald::layer()?;
        registry().with(filter).with(journal_layer).init();
    } else {
        fmt().with_env_filter(filter).init();
    }
    Ok(())
}

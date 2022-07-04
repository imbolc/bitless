//! Log errors into journal
//! Use `journalctl -r` after you run it
use displaydoc::Display;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Display, Error)]
enum Error {
    /// cannot initialize logging
    LogInit(#[from] bitless::journal::Error),
    /// cannot open file: {1}
    OpenFile(#[source] std::io::Error, &'static str),
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    use bitless::{error::chain, journal};

    ensure_rust_log_env_var();
    journal::init(true).map_err(|e| {
        eprintln!("{}", chain(&e));
        e
    })?;
    println!("Run `journalctl -f` to see the log entry");
    run().map_err(|e| {
        error!("{}", chain(&e));
        e
    })
}

fn run() -> Result<()> {
    let path = "not-found.txt";
    std::fs::File::open(path).map_err(|e| Error::OpenFile(e, path))?;
    Ok(())
}

fn ensure_rust_log_env_var() {
    use std::env;
    let key = "RUST_LOG";
    if env::var(key).is_err() {
        env::set_var(key, "info");
    }
}

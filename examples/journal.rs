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

    journal::init(true).map_err(|e| {
        eprintln!("{}", chain(&e));
        e
    })?;
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

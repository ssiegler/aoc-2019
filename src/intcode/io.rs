use std::io::BufRead;
use std::num::ParseIntError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    ParseIntError { error: ParseIntError },
    IoError { error: std::io::Error },
    Utf8Error { error: Utf8Error },
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseIntError { error }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError { error }
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8Error { error }
    }
}

pub fn read_program() -> Result<Vec<i32>, Error> {
    std::io::stdin()
        .lock()
        .split(b',')
        .map(|bytes| {
            bytes.map_err(Error::from).and_then(|bytes| {
                std::str::from_utf8(&bytes)
                    .map_err(Error::from)
                    .and_then(|s| s.trim().parse::<i32>().map_err(Error::from))
            })
        })
        .collect()
}

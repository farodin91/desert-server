
use std::io::Error as IoError;
use std::result::Result as StdResult;


pub enum Error {
    Std,
    Io(IoError),
}

pub type Result<T> = StdResult<T, Error>;
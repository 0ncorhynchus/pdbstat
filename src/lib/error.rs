use std;
use chemfiles;

#[derive(Debug)]
pub enum Error {
    ChemFiles(chemfiles::Error),
    InvalidArgument,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<chemfiles::Error> for Error {
    fn from(err: chemfiles::Error) -> Self {
        Error::ChemFiles(err)
    }
}

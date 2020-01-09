use std::ffi::NulError;

pub enum Error {
    Nul(NulError),
    IO,
    C,
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error::Nul(err)
    }
}

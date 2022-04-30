use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;
pub type Error = Box<ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    AlreadyExist,
    NotFound,
    DataIsCorrupted,
	UnexpectedIoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::UnexpectedIoError(err).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn error_from_io() {
        let io_error: std::io::Error = std::io::ErrorKind::TimedOut.into();
        let error: Error = io_error.into();
        assert_eq!(format!("{:?}", error), "UnexpectedIoError(Kind(TimedOut))");
    }
}

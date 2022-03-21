use std::error;
use std::fmt;
/// The error type returned by methods in this crate.
pub enum Error<E> {
    /// Manager Errors
    Inner(E),
    /// Timeout
    Timeout,
    /// BadConn
    BadConn,
    /// The pool has been closed or dropped
    PoolClosed,
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Error<E> {
        Error::Inner(e)
    }
}

impl<E> fmt::Display for Error<E>
where
    E: fmt::Display + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Inner(ref err) => write!(f, "{}", err),
            Error::Timeout => write!(f, "Timed out in mobc"),
            Error::BadConn => write!(f, "Bad connection in mobc"),
            Error::PoolClosed => write!(f, "The pool is closed"),
        }
    }
}

impl<E> fmt::Debug for Error<E>
where
    E: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Inner(ref err) => write!(f, "{:?}", err),
            Error::Timeout => write!(f, "Timed out in mobc"),
            Error::BadConn => write!(f, "Bad connection in mobc"),
            Error::PoolClosed => write!(f, "The pool is closed"),
        }
    }
}

impl<E> error::Error for Error<E>
where
    E: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Inner(ref err) => Some(err),
            Error::Timeout => None,
            Error::BadConn => None,
            Error::PoolClosed => None,
        }
    }
}

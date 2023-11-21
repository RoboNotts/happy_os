use std::io;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    SerialInit(serialport::ErrorKind),
    DecodeError,
    IO(io::ErrorKind),
    CheckSumFail,
    ResponseError
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::SerialInit(e) => write!(f, "Error initialising serial connection: {:?}", e),
            ErrorKind::IO(e) => write!(f, "Error with serial I/O: {}", e),
            ErrorKind::CheckSumFail => write!(f, "Failed to validate checksum on recieved data"),
            ErrorKind::DecodeError => write!(f, "Failed to decode message data"),
            ErrorKind::ResponseError => write!(f, "REcieved an incorrect response to a message"),
        }
    }
}
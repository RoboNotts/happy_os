use std::io;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind
}

pub enum ErrorKind {
    SerialInit(serialport::Error),
    IO(io::Error),
    CheckSumFail
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        return self.kind;
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::SerialInit(e) => write!(f, "Error initialising serial connection: {}", e.kind()),
            ErrorKind::SerialIO(e) => write!(f, "Error with serial I/O: {}", e.kind()),
            ErrorKind::CheckSumFail => write!(f, "Failed to validate checksum on recieved data")
        }
    }
}
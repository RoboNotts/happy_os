use std::error::Error;
use std::fmt::{Display, Formatter};
use serialport::Error as SerialError;
use crate::message::ModbusResponseError;
use crate::motor_controller::motor_status::MotorStatusParseError;

#[derive(Debug)]
pub enum MotorControllerError {
    SerialError(SerialError),
    CheckSumFail,
    ResponseError(ModbusResponseError),
    IOError(std::io::Error),
    InvalidResponder,
    IncorrectDataLength(usize, usize),
    IncorrectResponseType,
    MotorStatusParseError(MotorStatusParseError),
}

impl Error for MotorControllerError {}

impl Display for MotorControllerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerialError(e) => write!(f, "Error initialising serial connection. Reason: {}", e),
            Self::IOError(e) => write!(f, "Error with I/O. Reason: {}", e),
            Self::CheckSumFail => write!(f, "Failed to validate checksum on received data"),
            Self::ResponseError(e) => write!(f, "Error in modbus response. Reason: {}", e),
            Self::InvalidResponder => write!(f, "Invalid client responded to host"),
            Self::IncorrectDataLength(expected, got) => write!(f, "Invalid response length. Expected {}, got {}", expected, got),
            Self::IncorrectResponseType => write!(f, "Incorrect response type"),
            Self::MotorStatusParseError(e) => write!(f, "Failed to parse motor status. Reason: {}", e)
        }
    }
}

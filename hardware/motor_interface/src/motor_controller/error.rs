use crate::message::{ModbusRegister, ModbusResponseError};
use crate::motor_controller::motor_status::MotorStatusParseError;
use serialport::Error as SerialError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MotorControllerError {
    #[error("Error initialising serial connection")]
    SerialError(#[from] SerialError),
    #[error("Could not validate response checksum")]
    CheckSumFail,
    #[error("Error in modbus response")]
    ResponseError(#[from] ModbusResponseError),
    #[error("Error while reading")]
    IOError(#[from] std::io::Error),
    #[error("Invalid client responded to host. Expected {0}, got {1}")]
    InvalidResponder(u8, u8),
    #[error("Expected data of length {0}, got {1}")]
    IncorrectDataLength(usize, usize),
    #[error("Incorrect response type")]
    IncorrectResponseType,
    #[error("Incorrect response register. Expected {0:?}, got {1:?}")]
    IncorrectResponseRegister(ModbusRegister, ModbusRegister),
    #[error("Expected value {0}, got {1}")]
    IncorrectResponseValue(u16, u16),
    #[error("Failed to parse motor status {0}")]
    MotorStatusParseError(#[from] MotorStatusParseError),
}

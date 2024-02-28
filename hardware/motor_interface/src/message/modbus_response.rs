use crate::crc::crc16;
use crate::message::modbus_command::{ModbusCommand, ModbusCommandParseError};
use crate::message::modbus_register::{ModbusRegister, ModbusRegisterParseError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;

// Response from the Devices
// For read commands
// | Device Address | Command | Data Length | Data Response High | ... | Data Response Low | CRC High | CRC LOW |
// For write commands the device should echo back to the master the command that was sent as confirmation
// | Device Address | Command | Register Address 1 | Register Address 2 | Register Value High | Register Value Low | CRC High | CRC LOW |
pub enum ModbusResponse {
    WriteMessage {
        device_address: u8,
        command: ModbusCommand,
        register: ModbusRegister, // Modbus Register Address
        value: u16,
    },
    ReadMessage {
        device_address: u8,
        command: ModbusCommand,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub enum ModbusResponseError {
    RegisterParseError(ModbusRegisterParseError),
    IOError(std::io::Error),
    CommandParseError(ModbusCommandParseError),
    CheckSumFail,
}

impl Display for ModbusResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModbusResponseError::RegisterParseError(e) => {
                write!(f, "Failed to parse response. Reason: {}", e)
            }
            ModbusResponseError::IOError(e) => {
                write!(f, "I/O issue when sending request. Reason: {}", e)
            }
            ModbusResponseError::CommandParseError(e) => {
                write!(f, "Failed to parse response. Reason: {}", e)
            }
            ModbusResponseError::CheckSumFail => write!(f, "Failed to validate response checksum"),
        }
    }
}

impl Error for ModbusResponseError {}

impl ModbusResponse {
    pub fn from_reader(buf: &mut dyn Read) -> Result<ModbusResponse, ModbusResponseError> {
        let mut message_start: [u8; 2] = [0; 2];

        buf.read_exact(&mut message_start)
            .map_err(|e| ModbusResponseError::IOError(e))?;

        let device_address = message_start[0];

        let command: ModbusCommand = message_start[1]
            .try_into()
            .map_err(|e| ModbusResponseError::CommandParseError(e))?;

        match command {
            ModbusCommand::WriteRegister => {
                let mut message_end: [u8; 6] = [0; 6];

                buf.read_exact(&mut message_end)
                    .map_err(|e| ModbusResponseError::IOError(e))?;

                let register: ModbusRegister = (((message_end[0] as u16) << 8)
                    + (message_end[1] as u16))
                    .try_into()
                    .map_err(|e| ModbusResponseError::RegisterParseError(e))?;

                let mut message_data: [u8; 6] = [0; 6];
                message_data[..2].copy_from_slice(&message_start);
                message_data[2..].copy_from_slice(&message_end);

                if crc16(&message_data) != (((message_end[4] as u16) << 8) + message_end[5] as u16)
                {
                    Err(ModbusResponseError::CheckSumFail)
                } else {
                    Ok(ModbusResponse::WriteMessage {
                        device_address,
                        command,
                        register,
                        value: ((message_end[2] as u16) << 8) | (message_end[3] as u16),
                    })
                }
            }
            ModbusCommand::ReadRegister => {
                let mut data_len_buf: [u8; 1] = [0; 1];

                buf.read_exact(&mut data_len_buf)
                    .map_err(|e| ModbusResponseError::IOError(e))?;

                let data_len = data_len_buf[0] as usize;
                let mut message_end: Vec<u8> = vec![0; data_len + 2];

                buf.read_exact(message_end.as_mut_slice())
                    .map_err(|e| ModbusResponseError::IOError(e))?;

                if crc16(&message_end[..(message_end.len() - 2)])
                    != (((message_end[message_end.len() - 2] as u16) << 8)
                        + message_end[message_end.len() - 1] as u16)
                {
                    Err(ModbusResponseError::CheckSumFail)
                } else {
                    Ok(ModbusResponse::ReadMessage {
                        device_address,
                        command,
                        data: message_end[..data_len].to_vec(),
                    })
                }
            }
            ModbusCommand::WriteLocation => todo!(),
            ModbusCommand::ChangeDeviceAddress => todo!(),
        }
    }
}

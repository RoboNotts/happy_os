use crate::crc::crc16;
use crate::message::modbus_command::{ModbusCommand, ModbusCommandParseError};
use crate::message::modbus_register::{ModbusRegister, ModbusRegisterParseError};
use std::fmt::Display;
use std::io::Read;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ModbusResponseError {
    #[error("Failed to parse response!")]
    RegisterParseError(#[from] ModbusRegisterParseError),
    #[error("I/O issue when sending request!")]
    IOError(#[from] std::io::Error),
    #[error("Failed to parse response!")]
    CommandParseError(ModbusCommandParseError),
    #[error("Failed to validate response checksum!")]
    CheckSumFail,
}

impl ModbusResponse {
    pub fn from_reader(buf: &mut dyn Read) -> Result<ModbusResponse, ModbusResponseError> {
        let mut message_start: [u8; 2] = [0; 2];

        buf.read_exact(&mut message_start)
            .map_err(ModbusResponseError::IOError)?;

        let device_address = message_start[0];

        let command: ModbusCommand = message_start[1]
            .try_into()
            .map_err(ModbusResponseError::CommandParseError)?;

        match command {
            ModbusCommand::WriteRegister => {
                let mut message_end: [u8; 6] = [0; 6];

                buf.read_exact(&mut message_end)
                    .map_err(ModbusResponseError::IOError)?;

                let register: ModbusRegister = (((message_end[0] as u16) << 8)
                    + (message_end[1] as u16))
                    .try_into()
                    .map_err(ModbusResponseError::RegisterParseError)?;

                let mut message_data: [u8; 8] = [0; 8];
                message_data[..2].copy_from_slice(&message_start);
                message_data[2..].copy_from_slice(&message_end);

                if crc16(&message_data[0..6]) != (((message_data[6] as u16) << 8) + message_data[7] as u16)
                {
                    Err(ModbusResponseError::CheckSumFail)
                } else
                {
                    Ok(ModbusResponse::WriteMessage {
                        device_address,
                        command,
                        register,
                        value: ((message_data[4] as u16) << 8) | (message_data[5] as u16),
                    })
                }
            }
            ModbusCommand::ReadRegister => {
                let mut data_len_buf: [u8; 1] = [0; 1];

                buf.read_exact(&mut data_len_buf)
                    .map_err(ModbusResponseError::IOError)?;

                let data_len = data_len_buf[0] as usize;
                let mut message_data: Vec<u8> = vec![0; 2 + 1 + data_len + 2];

                message_data[0..2].copy_from_slice(&message_start);
                message_data[2] = data_len_buf[0];

                buf.read_exact(&mut message_data[3..])
                    .map_err(ModbusResponseError::IOError)?;

                if crc16(&message_data[0..(message_data.len() - 2)])
                    != (((message_data[message_data.len() - 2] as u16) << 8)
                        + message_data[message_data.len() - 1] as u16)
                {
                    Err(ModbusResponseError::CheckSumFail)
                } else {
                    Ok(ModbusResponse::ReadMessage {
                        device_address,
                        command,
                        data: message_data[3..(3 + data_len)].to_vec(),
                    })
                }
            }
            ModbusCommand::WriteLocation => todo!(),
            ModbusCommand::ChangeDeviceAddress => todo!(),
            ModbusCommand::WritePulse => unimplemented!()
        }
    }
}

use crate::message::*;
use crate::motor_controller::error::MotorControllerError;
use crate::motor_controller::motor_status::MotorStatus;
use constants::{MOTOR_BAUD_RATE, MOTOR_CONNECTION_TIMEOUT, MOTOR_GEAR, MOTOR_WHEEL_LENGTH};
use serialport::SerialPort;
use std::io::Write;

mod constants;
pub mod error;
pub mod motor_status;

pub struct MotorController {
    device_address: u8,
    port: Box<dyn SerialPort>,
}

impl MotorController {
    pub fn new(
        port_path: &str,
        device_address: u8,
    ) -> Result<MotorController, MotorControllerError> {
        // Establish a connection to the motor port
        let port = serialport::new(port_path, MOTOR_BAUD_RATE)
            // Char size 8
            .data_bits(serialport::DataBits::Eight)
            // No parity bits
            .parity(serialport::Parity::None)
            // Stop bits
            .stop_bits(serialport::StopBits::One)
            // No hardware flow control
            .flow_control(serialport::FlowControl::None)
            // ---
            .timeout(MOTOR_CONNECTION_TIMEOUT)
            .open()
            .map_err(|e: serialport::Error| MotorControllerError::SerialError(e))?;

        Ok(MotorController {
            port,
            device_address,
        })
    }

    pub fn request(
        &mut self,
        message: &ModbusRequest,
    ) -> Result<ModbusResponse, MotorControllerError> {
        let frame = message.to_message_bytes();

        self.port
            .write_all(&frame)
            .map_err(MotorControllerError::IOError)?;
        self.port
            .flush()
            .map_err(MotorControllerError::IOError)?;

        let v = ModbusResponse::from_reader(&mut self.port)
            .map_err(MotorControllerError::ResponseError)?;

        match (message.command, &v) {
            (ModbusCommand::WriteLocation, _) => todo!(),
            (ModbusCommand::ChangeDeviceAddress, _) => todo!(),
            (ModbusCommand::ReadRegister, ModbusResponse::ReadMessage { device_address, .. }) => {
                if message.device_address != *device_address {
                    Err(MotorControllerError::InvalidResponder(message.device_address, *device_address))
                } else {
                    Ok(v)
                }
            }
            (
                ModbusCommand::WriteRegister,
                ModbusResponse::WriteMessage {
                    device_address,
                    register,
                    ..
                },
            ) => {
                if message.device_address != *device_address {
                    Err(MotorControllerError::InvalidResponder(self.device_address, message.device_address))
                } else if message.register != *register {
                    Err(MotorControllerError::IncorrectResponseRegister(*register, message.register))
                } else {
                    Ok(v)
                }
            }
            (_, _) => Err(MotorControllerError::IncorrectResponseType),
        }
    }

    pub fn enable_modbus(&mut self) -> Result<(), MotorControllerError> {
        let device_address = self.device_address;

        let modbus_enable_message: ModbusRequest = ModbusRequest {
            device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::EnableModbus,
            value: 0x1,
        };

        self.request(&modbus_enable_message)?;

        Ok(())
    }

    pub fn set_motor_enabled(&mut self) -> Result<(), MotorControllerError> {
        let set_motor_enabled_message = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::EnableMotor,
            value: 0x1,
        };

        self.request(&set_motor_enabled_message)?;

        Ok(())
    }

    pub fn set_motor_disabled(&mut self) -> Result<(), MotorControllerError> {
        let set_motor_enabled_message = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::EnableMotor,
            value: 0x0,
        };

        self.request(&set_motor_enabled_message)?;

        Ok(())
    }

    pub fn get_rpm(&mut self) -> Result<i16, MotorControllerError> {
        let get_velocity_message = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorCurrentSpeed,
            value: 0x1,
        };

        let resp: ModbusResponse = self.request(&get_velocity_message)?;

        match resp {
            ModbusResponse::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(MotorControllerError::IncorrectDataLength(2, data.len()))
                } else {
                    let rpm: i16 = (((data[0] as u16) << 8) | (data[1] as u16)) as i16;
                    Ok(rpm)
                }
            }
            _ => Err(MotorControllerError::IncorrectResponseType),
        }
    }

    pub fn get_velocity(&mut self) -> Result<f32, MotorControllerError> {
        let rpm = self.get_rpm()?;

        let speed: f32 = (rpm as f32) / 60.0 * MOTOR_WHEEL_LENGTH / MOTOR_GEAR as f32 / 10.0;

        Ok(speed)
    }

    pub fn set_rpm(&mut self, speed: i16) -> Result<(), MotorControllerError> {
        let set_velocity_message = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::MotorTargetSpeed,
            value: speed as u16,
        };

        println!("set_velocity_message: {:?}", set_velocity_message.to_message_bytes());

        let response = self.request(&set_velocity_message)?;

        match response {
            ModbusResponse::WriteMessage { device_address, register, value, .. } => {
                if device_address != self.device_address {
                    Err(MotorControllerError::InvalidResponder(self.device_address, device_address))
                } else if !matches!(register, ModbusRegister::MotorTargetSpeed) {
                    Err(MotorControllerError::IncorrectResponseRegister(ModbusRegister::MotorTargetSpeed, register))
                } else if value != speed as u16 {
                    Err(MotorControllerError::IncorrectResponseValue(speed as u16, value))
                } else {
                    Ok(())
                }
            }
            ModbusResponse::ReadMessage { .. } => {
                Err(MotorControllerError::IncorrectResponseType)
            }
        }
    }

    pub fn set_velocity(&mut self, speed: f32) -> Result<(), MotorControllerError> {
        let rpm: i16 = (speed * 60.0 / MOTOR_WHEEL_LENGTH * MOTOR_GEAR as f32 * 10.0) as i16;

        self.set_rpm(rpm)?;

        Ok(())
    }

    pub fn get_position(&mut self) -> Result<i32, MotorControllerError> {
        // DATA_LOW
        let get_position_low = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorAbsolutePositionLow,
            value: 0x1,
        };

        let resp_l: ModbusResponse = self.request(&get_position_low)?;

        let low = match resp_l {
            ModbusResponse::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(MotorControllerError::IncorrectDataLength(2, data.len()))
                } else {
                    let data_low: u16 = ((data[0] as u16) << 8) | (data[1] as u16);
                    Ok(data_low)
                }
            }
            _ => Err(MotorControllerError::IncorrectResponseType),
        }?;

        // DATA_HIGH
        let get_position_high = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorAbsolutePositionHigh,
            value: 0x1,
        };

        let resp_h: ModbusResponse = self.request(&get_position_high)?;

        let high = match resp_h {
            ModbusResponse::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(MotorControllerError::IncorrectDataLength(2, data.len()))
                } else {
                    let data_high: u16 = ((data[0] as u16) << 8) + (data[1] as u16);
                    Ok(data_high)
                }
            }
            _ => Err(MotorControllerError::IncorrectResponseType),
        }?;

        let data = (((high as u32) << 16) | low as u32) as i32;

        Ok(data)
    }

    // pub fn set_position(&mut self, position: i32) -> Result<(), Error> {
    //     // let mut message: [u8; 11] = MOTOR_SET_POSITION_MAGIC_FRAME;
    //     // message[7] = ((position as u32) >> 8) as u8;
    //     // message[8] = position as u8;
    //     // message[9] = ((position as u32) >> 24) as u8;
    //     // message[10] = ((position as u32) >> 16) as u8;

    //     // let _resp: [u8; 6] = self.request(&message)?;

    //     // Ok(())
    //     todo!()
    // }

    pub fn get_status(&mut self) -> Result<MotorStatus, MotorControllerError> {
        let get_status = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorAlarmCode,
            value: 0x1,
        };

        let resp: ModbusResponse = self.request(&get_status)?;

        match resp {
            ModbusResponse::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(MotorControllerError::IncorrectDataLength(2, data.len()))
                } else {
                    let code_raw: u16 = ((data[0] as u16) << 8) + (data[1] as u16);
                    let code: MotorStatus = code_raw
                        .try_into()
                        .map_err(MotorControllerError::MotorStatusParseError)?;
                    Ok(code)
                }
            }
            _ => Err(MotorControllerError::IncorrectResponseType),
        }
    }

    // Proportional scalar for the motor's speed afaik
    pub fn set_position_gain(&mut self, gain: i16) -> Result<(), MotorControllerError> {
        let set_pos_gain_message = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::MotorPositionLoopProportionalCoefficient,
            value: gain as u16,
        };

        self.request(&set_pos_gain_message)?;

        Ok(())
    }

    // Constant added to the motor's speed
    pub fn set_position_feedforward(&mut self, ff: i16) -> Result<(), MotorControllerError> {
        let set_pos_ff_message = ModbusRequest {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::MotorSpecialFunction, // This might be wrong?
            value: ff as u16,
        };

        self.request(&set_pos_ff_message)?;

        Ok(())
    }
}

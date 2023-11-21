use std::io::Write;
use crate::{Error, ErrorKind, constants::*};
use serialport::SerialPort;

pub struct MotorController {
    device_address: u8,
    port: Box<dyn SerialPort>
}

impl MotorController {
    pub fn new(port_path: &str, device_address: u8) -> Result<MotorController, Error> {
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
            .map_err(|e: serialport::Error| Error {
                kind: ErrorKind::SerialInit(e.kind()),
            })?;

        Ok(MotorController { port, device_address })
    }

    pub fn request(&mut self, message: &ModbusHostMessage) -> Result<ModbusWorkerMessage, Error> {
        let frame = message.to_message_bytes();

        self.port.write_all(&frame).map_err(|e| Error {
            kind: ErrorKind::IO(e.kind()),
        })?;
        self.port.flush().map_err(|e| Error {
            kind: ErrorKind::IO(e.kind()),
        })?;

        let v = ModbusWorkerMessage::from_reader(&mut self.port)?;

        match (message.command, &v) {
            (ModbusCommand::WriteLocation, _) => todo!(),
            (ModbusCommand::ChangeDeviceAddress, _) => todo!(),
            (ModbusCommand::ReadRegister, ModbusWorkerMessage::ReadMessage { device_address, .. }) => {
                if message.device_address != *device_address {
                    Err(Error { kind: ErrorKind::ResponseError })
                }
                else {
                    Ok(v)
                }
            },
            (ModbusCommand::WriteRegister, ModbusWorkerMessage::WriteMessage { device_address, register, .. }) => {
                if message.device_address != *device_address || message.register != *register {
                    Err(Error { kind: ErrorKind::ResponseError })
                }
                else {
                    Ok(v)
                }
            },
            (_, _) => Err(Error { kind: ErrorKind::ResponseError })
        }
    }

    pub fn enable_modbus(&mut self) -> Result<(), Error> {
        let device_address = self.device_address;

        let modbus_enable_message: ModbusHostMessage = ModbusHostMessage {
            device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::EnableModbus,
            value: 0x1
        };

        self.request(&modbus_enable_message)?;

        Ok(())
    }

    pub fn set_motor_enabled(&mut self) -> Result<(), Error> {
        let set_motor_enabled_message = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::EnableMotor,
            value: 0x1
        };

        self.request(&set_motor_enabled_message)?;
        
        Ok(())
    }

    pub fn set_motor_disabled(&mut self) -> Result<(), Error> {
        let set_motor_enabled_message = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::EnableMotor,
            value: 0x0
        };

        self.request(&set_motor_enabled_message)?;
        
        Ok(())
    }

    pub fn get_rpm(&mut self) -> Result<i16, Error> {
        let get_velocity_message = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorCurrentSpeed,
            value: 0x1
        };

        let resp: ModbusWorkerMessage = self.request(&get_velocity_message)?;

        match resp {
            ModbusWorkerMessage::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(Error {
                        kind: ErrorKind::ResponseError
                    })
                }
                else {
                    let rpm: i16 = (((data[0] as u16) << 8) | (data[1] as u16)) as i16;
                    Ok(rpm)
                }
            }
            _ => Err(Error {
                kind: ErrorKind::ResponseError
            }),
        }
    }

    pub fn get_velocity(&mut self) -> Result<f32, Error> {
        let rpm = self.get_rpm()?;

        let speed: f32 = (rpm as f32) / 60.0 * MOTOR_WHEEL_LENGTH / MOTOR_GEAR as f32 / 10.0;

        Ok(speed)
    }

    pub fn set_rpm(&mut self, speed: i16) -> Result<(), Error> {
        let set_velocity_message = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::MotorTargetSpeed,
            value: speed as u16
        };

        self.request(&set_velocity_message)?;
        
        Ok(())
    }

    pub fn set_velocity(&mut self, speed: f32) -> Result<(), Error> {

        let rpm: i16 = (speed * 60.0 / MOTOR_WHEEL_LENGTH * MOTOR_GEAR as f32 * 10.0) as i16;
        
        self.set_rpm(rpm)?;

        Ok(())
    }

    pub fn get_position(&mut self) -> Result<i32, Error> {
        // DATA_LOW
        let get_position_low = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorAbsolutePositionLow,
            value: 0x1
        };

        let resp_l: ModbusWorkerMessage = self.request(&get_position_low)?;
        
        let low = match resp_l {
            ModbusWorkerMessage::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(Error {
                        kind: ErrorKind::ResponseError
                    })
                }
                else {
                    let data_low: u16 = ((data[0] as u16) << 8) | (data[1] as u16);
                    Ok(data_low)
                }
            }
            _ => Err(Error {
                kind: ErrorKind::ResponseError
            }),
        }?;

        // DATA_HIGH
        let get_position_high = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorAbsolutePositionHigh,
            value: 0x1
        };

        let resp_h: ModbusWorkerMessage = self.request(&get_position_high)?;
        
        let high = match resp_h {
            ModbusWorkerMessage::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(Error {
                        kind: ErrorKind::ResponseError
                    })
                }
                else {
                    let data_high: u16 = ((data[0] as u16) << 8) + (data[1] as u16);
                    Ok(data_high)
                }
            }
            _ => Err(Error {
                kind: ErrorKind::ResponseError
            }),
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

    pub fn get_status(&mut self) -> Result<MotorStatus, Error> {
        let get_status = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::ReadRegister,
            register: ModbusRegister::MotorAbsolutePositionHigh,
            value: 0x1
        };

        let resp: ModbusWorkerMessage = self.request(&get_status)?;
        
        match resp {
            ModbusWorkerMessage::ReadMessage { data, .. } => {
                if data.len() != 2 {
                    Err(Error {
                        kind: ErrorKind::ResponseError
                    })
                }
                else {
                    let code_raw: u16 = ((data[0] as u16) << 8) + (data[1] as u16);
                    let code = MotorStatus::from_code(code_raw)
                        .ok_or(Error { kind: ErrorKind::DecodeError })?;
                    Ok(code)
                }
            }
            _ => Err(Error {
                kind: ErrorKind::ResponseError
            }),
        }
    }

    // Proportional scalar for the motor's speed afaik
    pub fn set_position_gain(&mut self, gain: i16) -> Result<(), Error> {
        let set_pos_gain_message = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::MotorPositionLoopProportianalCoefficeient,
            value: gain as u16
        };

        self.request(&set_pos_gain_message)?;
        
        Ok(())
    }

    // Constant added to the motor's speed
    pub fn set_position_feedforward(&mut self, ff: i16) -> Result<(), Error> {
        let set_pos_ff_message = ModbusHostMessage {
            device_address: self.device_address,
            command: ModbusCommand::WriteRegister,
            register: ModbusRegister::MotorSpecialFunction, // This might be wrong?
            value: ff as u16
        };

        self.request(&set_pos_ff_message)?;
        
        Ok(())
    }
}

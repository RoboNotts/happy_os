use std::{time::Duration, io::Read};

use crate::{crc::crc16, error};

pub type Error = error::Error;
pub type ErrorKind = error::ErrorKind;

#[derive(Debug)]
pub enum MotorStatus {
    None,
    Warning(MotorStatusWarning),
    Fatal(MotorStatusFatal),
}

#[derive(Debug)]
pub enum MotorStatusWarning {
    HighTemperature,
    FlashWriteFailed,
}

#[derive(Debug)]
pub enum MotorStatusFatal {
    Overheat,
    SystemStall,
    UnderVoltage,
    LoadTooHeavy
}

impl MotorStatus {
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::Fatal(_))
    }

    pub fn from_code(code: u16) -> Option<MotorStatus> {
        match code {
            0x11 => Some(Self::Fatal(MotorStatusFatal::Overheat)),
            0x12 => Some(Self::Fatal(MotorStatusFatal::SystemStall)),
            0x13 => Some(Self::Fatal(MotorStatusFatal::UnderVoltage)),
            0x14 => Some(Self::Fatal(MotorStatusFatal::LoadTooHeavy)),
            0x10 => Some(Self::Warning(MotorStatusWarning::HighTemperature)),
            0x20 => Some(Self::Warning(MotorStatusWarning::FlashWriteFailed)),
            0x0 => Some(Self::None),
            _ => None,
        }
    }
}

// Using the Modbus Protocol, you can write to the registers on the motors
// directly. The following Enum describes the functions that are available
// to the programmer
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ModbusCommand {
    ReadRegister, // 0x3
    WriteRegister, // 0x6
    WriteLocation, // 0x78
    ChangeDeviceAddress // 0x7a
}

impl ModbusCommand {
    pub fn get_code(&self) ->  u8 {
        match self {
            Self::ReadRegister => 0x3,
            Self::WriteRegister => 0x6,
            Self::WriteLocation => 0x78,
            Self::ChangeDeviceAddress => 0x7a
        }
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0x3 => Some(Self::ReadRegister),
            0x6 => Some(Self::WriteRegister),
            0x78 => Some(Self::WriteLocation),
            0x7a => Some(Self::ChangeDeviceAddress),
            _ => None
        }
    }
}

// These are the registers available over the Modbus
#[derive(PartialEq, Eq)]
pub enum ModbusRegister {
    // Set to 0 to disable
    // Set to 1 to enable
    EnableModbus, // 0x0
    // Set to 0 to disable
    // Set to 1 to enable
    EnableMotor, // 0x01
    // Target speed from 0~3000 Rotations per Minute
    MotorTargetSpeed, // 0x02
    // The motor generates an acceleration curve using this value from 0~60000 (Rotations per Minute) per Second
    // Above 60000 the motor ignores this value (the specsheet says it does not accelerate? behaviour unspecified)
    MotorAcceleration, // 0x03
    // Initial Motor speed 0~500 Rotations per Minute
    MotorInitialSpeed, // 0x04
    // Afaik this is a proportional scalar for the motor speed.
    // 0~10000 maps to 0.0~10.000
    MotorSpeedLoopProportionalCoefficeient, // 0x05
    // Afaik this is a time scale used to calculate the integral part for the motor speed.
    // 2~2000ms 
    MotorSpeedLoopIntegrationTime, // 0x06
    // Afaik this is a proportional scalar for the position of the motor
    // 60~5000
    MotorPositionLoopProportianalCoefficeient, //0x07
    // Speed FeedForward 0~8.0V/KRPM (i.e Voltage increase per 1k RPM of speed)
    // Unsure if this is a scaled int, or a f32
    MotorSpeedFeedForwardVoltage, // 0x08
    // Set to 0 to clockwise
    // Set to 1 to anti-clockwise
    MotorDirectionPolarity, // 0x09
    // Unsure if this is correct? Translated as "Electronic gear molecule"
    // 16-bits
    MotorElectronicGearHigh, // 0x0A
    MotorElectronicGearLow, // 0x0B
    // Read only target position of the motor
    // 16-bits
    MotorTargetPostionLow, // 0x0C
    MotorTargetPostionHigh,// 0x0D
    // Read-only Alarm code. See MotorStatus
    MotorAlarmCode, // 0x0E
    // Read only motor current 0~32767; Actual Current (A) = x/2000
    MotorI, // 0x0F
    // Read-only current speed -30000~30000 (Rotations per Minute)
    MotorCurrentSpeed, // 0x10
    // Read-only motor voltage 0~32767; Actual Voltage (V) = x/327
    MotorV, // 0x11
    // Read-only system temperature 0~100 degrees
    SystemTemperature, // 0x12
    // Read-only system PWM -32768~32767 maps -100%~100%
    SystemOutputPWM, //0x13
    // Parameter saving flag 0~1 for write 0~2 read
    // This allows a programmer to save a preset for the various parameters listed among the registers
    // 0: Parameters are not saved (or write to unset all parametes)
    // 1: Saving parameters (or write to save the current parameters)
    // 2: Saved (i.e. there are saved parameters on this device.)
    ParameterSavingFlag, // 0x14
    // Read-only get the address of this device on the bus 0~255
    DeviceAddress, //0x15
    // The absolute position of the motor. 16 bits which represents
    // the number of steps taken
    MotorAbsolutePositionLow, //0x16
    MotorAbsolutePositionHigh, //0x17
    // Speed Filter frequency? 1~2000, default 100
    MotorSpeedFilterFrequency, //0x18
    // 1: Automatically reverses until EN[?] has a conduction signal,
    // then automatically forward until the encoder Z[?] signals stop.
    // 2. Automatically reverse until EN[?] has a conduction signal and stop.
    // 3. Automatically reverse until the encoder Z[?] signal stops
    MotorSpecialFunction // 0x19
}

// Registers are 16 bit addresses, although they don't go that high
impl ModbusRegister {
    // Function to get the u8 code for the enum
    fn get_code(&self) -> u16 {
        match self {
            ModbusRegister::EnableModbus => 0x00,
            ModbusRegister::EnableMotor => 0x01,
            ModbusRegister::MotorTargetSpeed => 0x02,
            ModbusRegister::MotorAcceleration => 0x03,
            ModbusRegister::MotorInitialSpeed => 0x04,
            ModbusRegister::MotorSpeedLoopProportionalCoefficeient => 0x05,
            ModbusRegister::MotorSpeedLoopIntegrationTime => 0x06,
            ModbusRegister::MotorPositionLoopProportianalCoefficeient => 0x07,
            ModbusRegister::MotorSpeedFeedForwardVoltage => 0x08,
            ModbusRegister::MotorDirectionPolarity => 0x09,
            ModbusRegister::MotorElectronicGearHigh => 0x0A,
            ModbusRegister::MotorElectronicGearLow => 0x0B,
            ModbusRegister::MotorTargetPostionLow => 0x0C,
            ModbusRegister::MotorTargetPostionHigh => 0x0D,
            ModbusRegister::MotorAlarmCode => 0x0E,
            ModbusRegister::MotorI => 0x0F,
            ModbusRegister::MotorCurrentSpeed => 0x10,
            ModbusRegister::MotorV => 0x11,
            ModbusRegister::SystemTemperature => 0x12,
            ModbusRegister::SystemOutputPWM => 0x13,
            ModbusRegister::ParameterSavingFlag => 0x14,
            ModbusRegister::DeviceAddress => 0x15,
            ModbusRegister::MotorAbsolutePositionLow => 0x16,
            ModbusRegister::MotorAbsolutePositionHigh => 0x17,
            ModbusRegister::MotorSpeedFilterFrequency => 0x18,
            ModbusRegister::MotorSpecialFunction => 0x19
        }
    }

    fn from_code(code: u16) -> Option<Self> {
        match code {
            0x00 => Some(ModbusRegister::EnableModbus),
            0x01 => Some(ModbusRegister::EnableMotor),
            0x02 => Some(ModbusRegister::MotorTargetSpeed),
            0x03 => Some(ModbusRegister::MotorAcceleration),
            0x04 => Some(ModbusRegister::MotorInitialSpeed),
            0x05 => Some(ModbusRegister::MotorSpeedLoopProportionalCoefficeient),
            0x06 => Some(ModbusRegister::MotorSpeedLoopIntegrationTime),
            0x07 => Some(ModbusRegister::MotorPositionLoopProportianalCoefficeient),
            0x08 => Some(ModbusRegister::MotorSpeedFeedForwardVoltage),
            0x09 => Some(ModbusRegister::MotorDirectionPolarity),
            0x0A => Some(ModbusRegister::MotorElectronicGearHigh),
            0x0B => Some(ModbusRegister::MotorElectronicGearLow),
            0x0C => Some(ModbusRegister::MotorTargetPostionLow),
            0x0D => Some(ModbusRegister::MotorTargetPostionHigh),
            0x0E => Some(ModbusRegister::MotorAlarmCode),
            0x0F => Some(ModbusRegister::MotorI),
            0x10 => Some(ModbusRegister::MotorCurrentSpeed),
            0x11 => Some(ModbusRegister::MotorV),
            0x12 => Some(ModbusRegister::SystemTemperature),
            0x13 => Some(ModbusRegister::SystemOutputPWM),
            0x14 => Some(ModbusRegister::ParameterSavingFlag),
            0x15 => Some(ModbusRegister::DeviceAddress),
            0x16 => Some(ModbusRegister::MotorAbsolutePositionLow),
            0x17 => Some(ModbusRegister::MotorAbsolutePositionHigh),
            0x18 => Some(ModbusRegister::MotorSpeedFilterFrequency),
            0x19 => Some(ModbusRegister::MotorSpecialFunction),
            _ => None
        }
    }
}

// Commands sent to the Device is structured as such
// | Device Address | Command | Register Address High | Register Address Low | Register Value High | Register Value Low | CRC High | CRC LOW |
pub struct ModbusHostMessage {
    pub device_address: u8, // Normally 0x1
    pub command: ModbusCommand,
    pub register: ModbusRegister, // Modbus Register Address 
    pub value: u16
}

impl ModbusHostMessage {
    pub fn to_message_bytes(&self) -> [u8; 8] {
        let register_code = self.register.get_code();
        let mut message_bytes: [u8; 8] = [self.device_address, self.command.get_code(), (register_code << 8) as u8, register_code as u8, (self.value << 8) as u8, self.value as u8, 0x0, 0x0];

        let crc = crc16(&message_bytes[..6]);
        message_bytes[6] = (crc << 8) as u8;
        message_bytes[7] = crc as u8;

        message_bytes
    }
}

// Response from the Devices
// For read commands
// | Device Address | Command | Data Length | Data Response High | ... | Data Response Low | CRC High | CRC LOW |
// For write commands the device should echo back to the master the command that was sent as confirmation
// | Device Address | Command | Register Address 1 | Register Address 2 | Register Value High | Register Value Low | CRC High | CRC LOW |
pub enum ModbusWorkerMessage {
    WriteMessage {
        device_address: u8,
        command: ModbusCommand,
        register: ModbusRegister, // Modbus Register Address 
        value: u16,
    },
    ReadMessage {
        device_address: u8,
        command: ModbusCommand,
        data: Vec<u8>
    }
}

impl ModbusWorkerMessage {
    pub fn from_reader(buf : &mut dyn Read) -> Result<ModbusWorkerMessage, Error> {
        let mut message_start: [u8; 2] = [0; 2];

        buf.read_exact(&mut message_start)
            .map_err(|e| Error { kind: ErrorKind::IO(e.kind()) })?;

        let device_address = message_start[0];
        
        let command = ModbusCommand::from_code(message_start[1])
            .ok_or(Error { kind: ErrorKind::DecodeError })?;

        match command {
            ModbusCommand::WriteRegister => {
                let mut message_end: [u8; 6] = [0; 6];
        
                buf.read_exact(&mut message_end)
                    .map_err(|e| Error { kind: ErrorKind::IO(e.kind()) })?;

                let register = ModbusRegister::from_code(((message_end[0] as u16) << 8) + (message_end[1] as u16))
                    .ok_or(Error { kind: ErrorKind::DecodeError })?;

                let mut message_data: [u8; 6] = [0; 6];
                message_data[..2].copy_from_slice(&message_start);
                message_data[2..].copy_from_slice(&message_end);
                
                if crc16(&message_data) != (((message_end[4] as u16) << 8) + message_end[5] as u16) {
                    Err(Error { kind: ErrorKind::CheckSumFail })
                }
                else {
                    Ok(ModbusWorkerMessage::WriteMessage {
                        device_address,
                        command,
                        register,
                        value: ((message_end[2] as u16) << 8) | (message_end[3] as u16)
                    })
                }
            }
            ModbusCommand::ReadRegister => {
                let mut data_len_buf: [u8;1] = [0; 1];

                buf.read_exact(&mut data_len_buf)
                    .map_err(|e| Error { kind: ErrorKind::IO(e.kind()) })?;

                let data_len = data_len_buf[0] as usize;
                let mut message_end: Vec<u8> = vec![0; data_len + 2];

                buf.read_exact(message_end.as_mut_slice())
                    .map_err(|e| Error { kind: ErrorKind::IO(e.kind()) })?;

                if crc16(&message_end[..(message_end.len() - 2)]) != (((message_end[message_end.len() - 2] as u16) << 8) + message_end[message_end.len() - 1] as u16) {
                    Err(Error { kind: ErrorKind::CheckSumFail })
                }
                else {
                    Ok(ModbusWorkerMessage::ReadMessage {
                        device_address,
                        command,
                        data: message_end[..data_len].to_vec()})
                }
            },
            ModbusCommand::WriteLocation => todo!(),
            ModbusCommand::ChangeDeviceAddress => todo!(),
        }
    }
}


// MOTOR CONNECTION CONSTANTS
pub(crate) const MOTOR_BAUD_RATE: u32 = 19200;
pub(crate) const MOTOR_CONNECTION_TIMEOUT : Duration = Duration::from_millis(10);

// MOTOR MAGIC CONSTANTS
// PHYSICAL
pub(crate) const MOTOR_GEAR : u32 = 16;
pub(crate) const MOTOR_WHEEL_LENGTH : f32 = 0.5843362;
pub(crate) const MOTOR_ENCODER_COUNT : u32 = 4000;
pub(crate) const MOTOR_WHEEL_DIST : f32 = 0.48342;
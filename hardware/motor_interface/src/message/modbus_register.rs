use std::fmt::Display;

// These are the registers available over the Modbus
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum ModbusRegister {
    // Set to 0 to disable
    // Set to 1 to enable
    EnableModbus = 0x0,
    // Set to 0 to disable
    // Set to 1 to enable
    EnableMotor = 0x01,
    // Target speed from 0~3000 Rotations per Minute
    MotorTargetSpeed = 0x02,
    // The motor generates an acceleration curve using this value from 0~60000 (Rotations per Minute) per Second
    // Above 60000 the motor ignores this value (the specsheet says it does not accelerate? behaviour unspecified)
    MotorAcceleration = 0x03,
    // Initial Motor speed 0~500 Rotations per Minute
    MotorInitialSpeed = 0x04,
    // Afaik this is a proportional scalar for the motor speed.
    // 0~10000 maps to 0.0~10.000
    MotorSpeedLoopProportionalCoefficient = 0x05,
    // Afaik this is a timescale used to calculate the integral part for the motor speed.
    // 2~2000ms
    MotorSpeedLoopIntegrationTime = 0x06,
    // Afaik this is a proportional scalar for the position of the motor
    // 60~5000
    MotorPositionLoopProportionalCoefficient = 0x07,
    // Speed FeedForward 0~8.0V/KRPM (i.e. Voltage increase per 1k RPM of speed)
    // Unsure if this is a scaled int, or a f32
    MotorSpeedFeedForwardVoltage = 0x08,
    // Set to 0 to clockwise
    // Set to 1 to anti-clockwise
    MotorDirectionPolarity = 0x09,
    // Unsure if this is correct? Translated as "Electronic gear molecule"
    // 16-bits
    MotorElectronicGearHigh = 0x0A,
    MotorElectronicGearLow = 0x0B,
    // Read only target position of the motor
    // 16-bits
    MotorTargetPositionLow = 0x0C,
    MotorTargetPositionHigh = 0x0D,
    // Read-only Alarm code. See MotorStatus
    MotorAlarmCode = 0x0E,
    // Read only motor current 0~32767; Actual Current (A) = x/2000
    MotorI = 0x0F,
    // Read-only current speed -30000~30000 (Rotations per Minute)
    MotorCurrentSpeed = 0x10,
    // Read-only motor voltage 0~32767; Actual Voltage (V) = x/327
    MotorV = 0x11,
    // Read-only system temperature 0~100 degrees
    SystemTemperature = 0x12,
    // Read-only system PWM -32768~32767 maps -100%~100%
    SystemOutputPWM = 0x13,
    // Parameter saving flag 0~1 for write 0~2 read
    // This allows a programmer to save a preset for the various parameters listed among the registers
    // 0: Parameters are not saved (or write to unset all parameters)
    // 1: Saving parameters (or write to save the current parameters)
    // 2: Saved (i.e. there are saved parameters on this device.)
    ParameterSavingFlag = 0x14,
    // Read-only get the address of this device on the bus 0~255
    DeviceAddress = 0x15,
    // The absolute position of the motor. 16 bits which represents
    // the number of steps taken
    MotorAbsolutePositionLow = 0x16,
    MotorAbsolutePositionHigh = 0x17,
    // Speed Filter frequency? 1~2000, default 100
    MotorSpeedFilterFrequency = 0x18,
    // 1: Automatically reverses until EN[?] has a conduction signal,
    // then automatically forward until the encoder Z[?] signals stop.
    // 2. Automatically reverse until EN[?] has a conduction signal and stop.
    // 3. Automatically reverse until the encoder Z[?] signal stops
    MotorSpecialFunction = 0x19,
}

#[derive(Debug)]
pub struct ModbusRegisterParseError;

impl std::error::Error for ModbusRegisterParseError {}

impl Display for ModbusRegisterParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to parse modbus register!")
    }
}

impl TryFrom<u16> for ModbusRegister {
    type Error = ModbusRegisterParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ModbusRegister::EnableModbus),
            0x01 => Ok(ModbusRegister::EnableMotor),
            0x02 => Ok(ModbusRegister::MotorTargetSpeed),
            0x03 => Ok(ModbusRegister::MotorAcceleration),
            0x04 => Ok(ModbusRegister::MotorInitialSpeed),
            0x05 => Ok(ModbusRegister::MotorSpeedLoopProportionalCoefficient),
            0x06 => Ok(ModbusRegister::MotorSpeedLoopIntegrationTime),
            0x07 => Ok(ModbusRegister::MotorPositionLoopProportionalCoefficient),
            0x08 => Ok(ModbusRegister::MotorSpeedFeedForwardVoltage),
            0x09 => Ok(ModbusRegister::MotorDirectionPolarity),
            0x0A => Ok(ModbusRegister::MotorElectronicGearHigh),
            0x0B => Ok(ModbusRegister::MotorElectronicGearLow),
            0x0C => Ok(ModbusRegister::MotorTargetPositionLow),
            0x0D => Ok(ModbusRegister::MotorTargetPositionHigh),
            0x0E => Ok(ModbusRegister::MotorAlarmCode),
            0x0F => Ok(ModbusRegister::MotorI),
            0x10 => Ok(ModbusRegister::MotorCurrentSpeed),
            0x11 => Ok(ModbusRegister::MotorV),
            0x12 => Ok(ModbusRegister::SystemTemperature),
            0x13 => Ok(ModbusRegister::SystemOutputPWM),
            0x14 => Ok(ModbusRegister::ParameterSavingFlag),
            0x15 => Ok(ModbusRegister::DeviceAddress),
            0x16 => Ok(ModbusRegister::MotorAbsolutePositionLow),
            0x17 => Ok(ModbusRegister::MotorAbsolutePositionHigh),
            0x18 => Ok(ModbusRegister::MotorSpeedFilterFrequency),
            0x19 => Ok(ModbusRegister::MotorSpecialFunction),
            _ => Err(Self::Error {}),
        }
    }
}

// Registers are 16 bit addresses, although they don't go that high
impl From<ModbusRegister> for u16 {
    fn from(value: ModbusRegister) -> Self {
        value as u16
    }
}

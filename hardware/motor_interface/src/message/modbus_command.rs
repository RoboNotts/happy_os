use std::fmt::Display;

// Using the Modbus Protocol, you can write to the registers on the motors
// directly. The following Enum describes the functions that are available
// to the programmer
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ModbusCommand {
    ReadRegister = 0x3,
    WriteRegister = 0x6,
    WriteLocation = 0x78,
    ChangeDeviceAddress = 0x7a
}

#[derive(Debug)]
pub struct ModbusCommandParseError;

impl std::error::Error for ModbusCommandParseError {}

impl Display for ModbusCommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to parse motor status!")
    }
}

impl TryFrom<u8> for ModbusCommand {
    type Error = ModbusCommandParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x3 => Ok(Self::ReadRegister),
            0x6 => Ok(Self::WriteRegister),
            0x78 => Ok(Self::WriteLocation),
            0x7a => Ok(Self::ChangeDeviceAddress),
            _ => Err(Self::Error {})
        }
    }
}


impl From<ModbusCommand> for u8 {
    fn from(value: ModbusCommand) -> Self {
        value as u8
    }
}

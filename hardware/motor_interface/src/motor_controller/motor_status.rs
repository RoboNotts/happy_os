use std::fmt::Display;

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
    LoadTooHeavy,
}

impl MotorStatus {
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::Fatal(_))
    }
}

#[derive(Debug)]
pub struct MotorStatusParseError;

impl std::error::Error for MotorStatusParseError {}

impl Display for MotorStatusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to parse motor status!")
    }
}

impl TryFrom<u16> for MotorStatus {
    type Error = MotorStatusParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x11 => Ok(Self::Fatal(MotorStatusFatal::Overheat)),
            0x12 => Ok(Self::Fatal(MotorStatusFatal::SystemStall)),
            0x13 => Ok(Self::Fatal(MotorStatusFatal::UnderVoltage)),
            0x14 => Ok(Self::Fatal(MotorStatusFatal::LoadTooHeavy)),
            0x10 => Ok(Self::Warning(MotorStatusWarning::HighTemperature)),
            0x20 => Ok(Self::Warning(MotorStatusWarning::FlashWriteFailed)),
            0x0 => Ok(Self::None),
            _ => Err(Self::Error {}),
        }
    }
}

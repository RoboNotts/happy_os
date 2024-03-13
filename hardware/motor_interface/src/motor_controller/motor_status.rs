
use thiserror::Error;

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

#[derive(Debug, Error)]
#[error("Failed to parse motor status! {0}")]
pub struct MotorStatusParseError(u16);

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
            a => Err(MotorStatusParseError(a)),
        }
    }
}

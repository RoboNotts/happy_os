use std::time::Duration;

// MOTOR CONNECTION CONSTANTS
pub(crate) const MOTOR_BAUD_RATE: u32 = 115200;
pub(crate) const MOTOR_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

// MOTOR MAGIC CONSTANTS
// PHYSICAL
pub(crate) const MOTOR_GEAR: u32 = 16;
pub(crate) const MOTOR_WHEEL_LENGTH: f32 = 0.5843362;
pub(crate) const MOTOR_ENCODER_COUNT: u32 = 4000;
pub(crate) const MOTOR_WHEEL_DIST: f32 = 0.48342;

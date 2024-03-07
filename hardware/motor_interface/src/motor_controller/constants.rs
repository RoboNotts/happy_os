use std::time::Duration;

// MOTOR CONNECTION CONSTANTS
pub(super) const MOTOR_BAUD_RATE: u32 = 19_200;
pub(super) const MOTOR_CONNECTION_TIMEOUT: Duration = Duration::from_secs(1);

// MOTOR MAGIC CONSTANTS
// PHYSICAL
pub(super) const MOTOR_GEAR: u32 = 16;
pub(super) const MOTOR_WHEEL_LENGTH: f32 = 0.5843362;
pub(super) const MOTOR_ENCODER_COUNT: u32 = 4000;
pub(super) const MOTOR_WHEEL_DIST: f32 = 0.48342;

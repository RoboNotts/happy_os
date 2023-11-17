// MOTOR CONNECTION CONSTANTS
// PRIVATE
pub(crate) const MOTOR_BAUD_RATE: u32 = 19200;

pub(crate) const MOTOR_CONNECTION_TIMEOUT : Duration = Duration::from_millis(10);

// PUBLIC
pub(crate) const LEFT_SERVO_NAME : &str = "amy_485_port_left";

pub(crate) const RIGHT_SERVO_NAME: &str = "amy_485_port_right";

// MOTOR MAGIC CONSTANTS
// DIGITAL
pub(crate) const MOTOR_GET_VELOCITY_MAGIC_FRAME: [u8; 6] = [0x01,0x03,0x00,0x10,0x0,0x1];
pub(crate) const MOTOR_SET_VELOCITY_MAGIC_FRAME: [u8; 6] = [0x01,0x06,0x00,0x02,0x0,0x0];
pub(crate) const MOTOR_GET_POSITION_L_MAGIC_FRAME: [u8; 6] = [0x01,0x03,0x0,0x16,0x0,0x01];
pub(crate) const MOTOR_GET_POSITION_H_MAGIC_FRAME: [u8; 6] = [0x01,0x03,0x0,0x17,0x0,0x01];
pub(crate) const MOTOR_SET_POSITION_MAGIC_FRAME: [u8; 11] = [0x01,0x10,0x0,0x0c,0x0,0x02,0x04,0x0,0x0,0x0,0x0];
pub(crate) const MOTOR_GET_STATUS_MAGIC_FRAME: [u8; 6] = [0x01,0x03,0x0,0x0E,0x0,0x01];
pub(crate) const MOTOR_SET_POSTION_GAIN_MAGIC_FRAME : [u8; 6] = [0x01,0x06,0x00,0x07,0x0,0x0];
pub(crate) const MOTOR_SET_POSTION_FF_MAGIC_FRAME : [u8; 6] = [0x01,0x06,0x00,0x19,0x0,0x0];

pub(crate) const MOTOR_DISABLE_MAGIC_FRAME: [u8; 8] = [0x01, 0x06, 0x0, 0x1, 0x0, 0x0, 0xd8, 0x0a];
pub(crate) const MOTOR_ENABLE_MAGIC_FRAME: [u8; 8] = [0x01, 0x06, 0x0, 0x1, 0x0, 0x01, 0x19, 0xca];


// PHYSICAL
pub(crate) const MOTOR_GEAR : u32 = 16;

pub(crate) const MOTOR_WHEEL_LENGTH : f32 = 0.5843362;

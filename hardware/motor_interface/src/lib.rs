pub(crate) mod error;
pub(crate) mod motor_controller;
pub(crate) mod constants;
pub(crate) mod crc;

use motor_controller::*;
use std::ffi::{CStr, c_char};

// Public types for errors
pub type Error = error::Error;
pub type ErrorKind = error::ErrorKind;

// Public FFI Constants
// extern "C" {
//     pub static LEFT_SERVO_NAME : &str = constants::LEFT_SERVO_NAME;
//     pub const RIGHT_SERVO_NAME : &str = constants::RIGHT_SERVO_NAME;
//     pub const DEFAULT_DEVICE_ADDRESS : u8 = constants::DEFAULT_DEVICE_ADDRESS;
// }

#[no_mangle]
pub unsafe extern "C" fn motor_controller_new(port_path: *mut c_char, device_address: u8) -> *mut MotorController {
    let port_path_cstr = unsafe {
        assert!(!port_path.is_null());
        CStr::from_ptr(port_path)
    };

    let port_path_rusty = port_path_cstr.to_str().unwrap();

    let mc = MotorController::new(port_path_rusty, device_address)
        .unwrap();

    Box::into_raw(Box::new(mc))
}

#[no_mangle]
pub unsafe extern "C" fn motor_controller_enable_modbus(ptr: *mut MotorController) -> () {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.enable_modbus()
        .unwrap();
}
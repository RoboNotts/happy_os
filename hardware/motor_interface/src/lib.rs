pub(crate) mod error;
pub mod motor_controller;
pub(crate) mod constants;
pub(crate) mod crc;

use motor_controller::*;
use std::ffi::{CStr, c_char};

// Public types for errors
pub type Error = error::Error;
pub type ErrorKind = error::ErrorKind;

// Public FFI Shims

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
pub unsafe extern "C" fn motor_controller_free(ptr: *mut MotorController) {
    if ptr.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(ptr) });
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

#[no_mangle]
pub unsafe extern "C" fn motor_controller_set_motor_enabled(ptr: *mut MotorController) {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.set_motor_enabled()
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn motor_controller_set_motor_disabled(ptr: *mut MotorController) {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.set_motor_disabled()
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn motor_controller_get_position(ptr: *mut MotorController) -> i32 {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.get_position()
        .unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn motor_controller_get_velocity(ptr: *mut MotorController) -> f32 {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.get_velocity()
        .unwrap()
}

pub unsafe extern "C" fn motor_controller_set_velocity(ptr: *mut MotorController, speed: f32) -> () {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.set_velocity(speed)
        .unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn motor_controller_set_position_feedforward(ptr: *mut MotorController, ff : i16) {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.set_position_feedforward(ff)
        .unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn motor_controller_set_position_gain(ptr: *mut MotorController, gain: i16) {
    let motor_controller = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    motor_controller.set_position_gain(gain)
        .unwrap()
}

#[cfg(test)]
mod tests;
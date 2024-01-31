use super::*;
use std::{thread::sleep as zzz, time::Duration};
use motor_controller::MotorController;

#[test]
fn this_tests_motors() {
    let mut controller = MotorController::new("/dev/ttyS1", 0x0)
        .unwrap_or_else(|e| panic!("Failed to set up Motor Controller! {}", e.kind()));

    controller.enable_modbus().unwrap_or_else(|e| panic!("Failed to enable modbus mode! {}", e.kind()));
    let unenabled_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e.kind()));
    println!("Motor Status: {:?}", unenabled_status);

    controller.set_motor_enabled().unwrap_or_else(|e| panic!("Failed to enable motor! {}", e.kind()));
    
    let enabled_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e.kind()));
    println!("Motor Status: {:?}", enabled_status);

    controller.set_velocity(0.1).unwrap_or_else(|e| panic!("Failed to set velocity! {}", e.kind()));

    zzz(Duration::from_secs(5));
    
    let running_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e.kind()));
    println!("Motor Status: {:?}", running_status);
    
    let running_speed = controller.get_velocity().unwrap_or_else(|e| panic!("Failed to get velocity! {}", e.kind()));
    println!("Motor Speed: {:?}", running_speed);

    controller.set_velocity(0.1).unwrap_or_else(|e| panic!("Failed to set velocity! {}", e.kind()));
    
    let final_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e.kind()));
    println!("Motor Status: {:?}", final_status);

    controller.set_motor_disabled().unwrap_or_else(|e| panic!("Failed to disable motor! {}", e.kind()));
    println!("HELL YES!");
}
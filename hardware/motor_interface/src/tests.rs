use super::*;
use motor_controller::MotorController;
use std::{thread::sleep as zzz, time::Duration};

mod magic_strings;

const MOTOR_PATH : &str = "/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_AB0KJXLJ-if00-port0";
// const MOTOR_PATH : &str = "/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_AB0KJDBY-if00-port0";

#[test]
fn this_tests_motors() {
    // NB When testing motors, both dip switches must be in the ON position (down).
    
    let mut controller = MotorController::new(MOTOR_PATH, 0x01)
        .unwrap_or_else(|e| {
            let ports = serialport::available_ports().expect("No ports found!");
            for p in ports {
                println!("{}", p.port_name);
            }

            panic!("Failed to set up Motor Controller! {}", e);
        });

    controller
        .enable_modbus()
        .unwrap_or_else(|e| panic!("Failed to enable modbus mode! {}", e));

    let unenabled_status = controller
        .get_status()
        .unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Unenabled Motor Status: {:?}", unenabled_status);

    controller
        .set_motor_enabled()
        .unwrap_or_else(|e| panic!("Failed to enable motor! {}", e));

    let enabled_status = controller
        .get_status()
        .unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Enabled Motor Status: {:?}", enabled_status);

    let still_speed = controller
        .get_velocity()
        .unwrap_or_else(|e| panic!("Failed to get velocity! {}", e));
    println!("Still Motor Speed: {:?}", still_speed);

    controller
        .set_velocity(0.05)
        .unwrap_or_else(|e| panic!("Failed to set rpm! {}", e));

    println!("RUN!");

    zzz(Duration::from_secs(1));

    let running_status = controller
        .get_status()
        .unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Motor Status: {:?}", running_status);

    let running_speed = controller
        .get_rpm()
        .unwrap_or_else(|e| panic!("Failed to get velocity! {}", e));
    println!("Motor RPM: {:?}", running_speed);
    
    controller
        .set_velocity(-0.05)
        .unwrap_or_else(|e| panic!("Failed to set rpm! {}", e));

    zzz(Duration::from_secs(1));

    println!("STOP!");

    controller
        .set_velocity(0.0)
        .unwrap_or_else(|e| panic!("Failed to set velocity! {}", e));

    let running_speed = controller
        .get_velocity()
        .unwrap_or_else(|e| panic!("Failed to get velocity! {}", e));
    println!("Final Motor Speed: {:?}", running_speed);

    let final_status = controller
        .get_status()
        .unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Final Motor Status: {:?}", final_status);

    // controller
    //     .set_motor_disabled()
    //     .unwrap_or_else(|e| panic!("Failed to disable motor! {}", e));
    // println!("HELL YES!");
}

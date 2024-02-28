use super::*;
use std::{thread::sleep as zzz, time::Duration};
use motor_controller::MotorController;

mod magic_strings;

// const MOTOR_PATH : &str = "/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_AB0KJXLJ-if00-port0";
// const MOTOR_PATH : &str = "/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_AB0KJDBY-if00-port0";

#[test]
fn this_tests_motors() {
    let available_ports = serialport::available_ports()
        .expect("Failed to get available ports");
    let mut buf = String::new();
    println!("Select port for testing:");
    for (i, p) in available_ports.iter().enumerate() {
        println!("{i}) {}", p.port_name)
    }
    println!("Enter below:");
    std::io::stdin().read_line(&mut buf).expect("Failed to read line from stdin");

    let chosen_one = available_ports.get(buf.trim().parse::<usize>().expect("Invalid input")).expect("Invalid port number").port_name.as_str();

    let mut controller = MotorController::new(chosen_one, 0x1)
        .unwrap_or_else(|e| panic!("Failed to set up Motor Controller! {}", e));

    controller.enable_modbus().unwrap_or_else(|e| panic!("Failed to enable modbus mode! {}", e));
    let unenabled_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Motor Status: {:?}", unenabled_status);

    controller.set_motor_enabled().unwrap_or_else(|e| panic!("Failed to enable motor! {}", e));
    
    let enabled_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Motor Status: {:?}", enabled_status);

    controller.set_velocity(0.1).unwrap_or_else(|e| panic!("Failed to set velocity! {}", e));

    zzz(Duration::from_secs(5));
    
    let running_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Motor Status: {:?}", running_status);
    
    let running_speed = controller.get_velocity().unwrap_or_else(|e| panic!("Failed to get velocity! {}", e));
    println!("Motor Speed: {:?}", running_speed);

    controller.set_velocity(0.1).unwrap_or_else(|e| panic!("Failed to set velocity! {}", e));
    
    let final_status = controller.get_status().unwrap_or_else(|e| panic!("Failed to get status! {}", e));
    println!("Motor Status: {:?}", final_status);

    controller.set_motor_disabled().unwrap_or_else(|e| panic!("Failed to disable motor! {}", e));
    println!("HELL YES!");
}

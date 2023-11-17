use serialport::{available_ports, SerialPort};

mod crc;
mod error;

pub type Error = error::Error;
pub type ErrorKind = error::ErrorKind;

mod constants;
use constants::*;
pub const LEFT_SERVO_NAME : &str = constants::LEFT_SERVO_NAME;
pub const RIGHT_SERVO_NAME : &str = constants::RIGHT_SERVO_NAME;

trait MotorController {
    pub fn enable(&self) -> Result<(), Error>;
    pub fn disable(&self) -> Result<(), Error>;
    pub fn send_raw<const N: usize>(&self, message: &[u8; N]) -> Result<(), Error>;
    pub fn recieve_raw<const N: usize>(&self) -> Result<[u8; N], Error>;
}

pub struct ModbusDisabledMotorController {
    port: Box<dyn SerialPort>,
}

impl ModbusDisabledMotorController {
    pub fn new(port_path: &str) -> Result<MotorController, Error> {
        // Establish a connection to the motor port
        let port = serialport::new(port_path, modname::MOTOR_BAUD_RATE)
            // Char size 8
            .data_bits(serialport::DataBits::Eight)
            // No parity bits
            .parity(serialport::Parity::None)
            // Stop bits
            .stop_bits(serialport::StopBits::One)
            // No hardware flow control
            .flow_control(serialport::FlowControl::None)
            // ---
            .timeout(modname::MOTOR_CONNECTION_TIMEOUT)
            .open()
            .map_err(|e: serialport::Error| Error {
                kind: ErrorKind::SerialInit(e),
            })?;

        Ok(MotorController { port })
    }

    pub fn enable_modbus(self) -> Result<ModbusEnabledMotorController, Error> {
        let port = self.port;
        port.write_all(&MODBUS_ENABLE_MAGIC_FRAME)
            .map_err(|e: std::io::Error| ErrorKind::SerialIO(e))?;
        // Block until the port is fully written to
        port.flush().map_err(|e| ErrorKind::SerialIO(e))?;

        let mut resp: [u8; 8] = [0; 8];
        port.read_exact(&resp).map_err(|e| ErrorKind::SerialIO(e))?;

        Ok(ModbusEnabledMotorController { port })
    }
}

impl MotorController for ModbusDisabledMotorController {
    fn enable(&self) {
        self.send_raw(&modname::MOTOR_ENABLE_MAGIC_FRAME);
    }

    fn disable(&self) {
        self.send_raw(&modname::MOTOR_DISABLE_MAGIC_FRAME)
    }

    fn send_raw<const N: usize>(&self, &message: [u8; N]) {
        self.port.write_all(&message).map_err(|e| Error {
            kind: error::ErrorKind::IO(e),
        });
        self.port.flush().map_err(|e| Error {
            kind: error::ErrorKind::IO(e),
        });
    }

    fn recieve_raw<const N: usize>(&self) -> [u8; N] {
        let mut resp: [u8; N] = [0; N];
        self.port.read_exact(&resp).map_err(|e| Error {
            kind: error::ErrorKind::IO(e),
        });

        resp
    }
}

pub struct ModbusEnabledMotorController {
    port: Box<dyn SerialPort>,
}

impl MotorController for ModbusEnabledMotorController {
    fn enable(&self) {
        self.send_raw(&modname::MOTOR_ENABLE_MAGIC_FRAME);
    }

    fn disable(&self) {
        self.send_raw(&modname::MOTOR_DISABLE_MAGIC_FRAME)
    }

    fn send_raw<const N: usize>(&self, &message: [u8; N]) {
        self.port.write_all(&message).map_err(|e| Error {
            kind: error::ErrorKind::IO(e),
        });
        self.port.flush().map_err(|e| Error {
            kind: error::ErrorKind::IO(e),
        });
    }

    fn recieve_raw<const N: usize>(&self) -> [u8; N] {
        let mut resp: [u8; N] = [0; N];
        self.port.read_exact(&resp).map_err(|e| Error {
            kind: error::ErrorKind::IO(e),
        });

        resp
    }
}

impl ModbusEnabledMotorController {
    pub fn request<const N: usize, const M: usize>(&self, message: &[u8; N]) -> Result<[u8; M], Error> {
        let mut frame: [u8; N + 2];
        message.clone_into(frame);
        let crc: u16 = crc::crc16(&message);
        frame[N] = ((crc >> 8) as u8) & 0xFF;
        frame[N + 1] = (crc & 0xFF) as u8;

        self.send_raw(&frame)?;
        let resp: [u8; M + 2] = self.recieve_raw();

        let res_crc = crc::crc16(&resp[..(M - 1)]);

        if res_crc != ((resp[M] as u16) << 8) | (resp[M + 1] as u16) {
            Error {
                kind: ErrorKind::CheckSumFail,
            }
        } else {
            Ok(resp[..(M - 1)] as [u8; M])
        }
    }

    pub fn get_rpm(&self) -> Result<i16, Error> {
        let resp: [u8; 5] = self.request(&MOTOR_GET_VELOCITY_MAGIC_FRAME)?;
        
        let rpm: i16 = (resp[3] as u16) << 8 | (resp[4] as u16);
        Ok(rpm)
    }

    pub fn get_velocity(&self) -> Result<f32, Error> {
        let rpm = self.get_rpm()?;

        let speed: f32 = rpm / 60.0 * modname::MOTOR_WHEEL_LENGTH / modname::MOTOR_GEAR / 10.0;

        Ok(speed)
    }

    pub fn set_velocity(&self, speed: i16) -> Result<(), Error> {
        let mut message: [u8; 6] = MOTOR_SET_VELOCITY_MAGIC_FRAME.clone();
        message[4] = (speed1 >> 8) as u8;
        message[5] = (speed1 & 0xFF) as u8;

        let resp: [u8; 5] = self.request(&message)?;
        
        // I guess it works?
        Ok(())
    }

    pub fn get_position(&self) -> Result<i32, Error> {
        // DATA_LOW
        let resp_1: [u8; 5] = self.request(&MOTOR_GET_POSITION_L_MAGIC_FRAME)?;

        let data_l = (resp_l[3] as u16) << 8 | (resp_l[4] as u16);

        // DATA_HIGH
        let resp_h: [u8; 5] = self.request(&MOTOR_GET_POSITION_H_MAGIC_FRAME)?;

        let data_h = (resp_h[3] as u16) << 8 | (resp_h[4] as u16);

        let data = (((data_h1 as u32) << 16) | data_l1 as u32) as i32;

        Ok(data)
    }

    pub fn set_position(&self, position: i32) -> Result<(), Error> {
        let message: [u8; 11] = modname::MOTOR_SET_POSITION_MAGIC_FRAME;
        message[7] = ((position as u32) >> 8) as u8;
        message[8] = position as u8;
        message[9] = ((position as u32) >> 24) as u8;
        message[10] = ((position as u32) >> 16) as u8;

        let resp: [u8; 6] = self.request(&message)?;

        Ok(())
    }

    pub fn get_status(&self) -> Result<u16, Error> {
        let resp: [u8; 5] = self.request(&MOTOR_GET_STATUS_MAGIC_FRAME)?;

        let error: u16 = (resp[3] as u16) << 8 | (resp[4] as u16);
        Ok(error)
    }

    // Proportional scalar for the motor's speed afaik
    pub fn set_position_gain(&self, gain: i16) -> Result<(), Error> {
        let message : [u8; 6] = MOTOR_SET_POSTION_GAIN_MAGIC_FRAME.clone();
        message[4] = (gain >> 8) as u8;
        message[5] = gain as u8;

        let resp : [u8; 6] = self.request(&message);

        Ok(())
    }

    // Constant added to the motor's speed
    pub fn set_position_feedforward(&self, gain: i16) -> Result<(), Error> {
        let message : [u8; 6] = MOTOR_SET_POSTION_GAIN_MAGIC_FRAME.clone();
        message[4] = (gain >> 8) as u8;
        message[5] = gain as u8;

        let resp : [u8; 6] = self.request(&message);
        
        Ok(())
    }
}

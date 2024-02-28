use crate::crc::crc16;
use crate::message::modbus_command::ModbusCommand;
use crate::message::modbus_register::ModbusRegister;

// Commands sent to the Device is structured as such
// | Device Address | Command | Register Address High | Register Address Low | Register Value High | Register Value Low | CRC High | CRC LOW |
pub struct ModbusRequest {
    pub device_address: u8, // Normally 0x1
    pub command: ModbusCommand,
    pub register: ModbusRegister, // Modbus Register Address
    pub value: u16
}

impl ModbusRequest {
    pub fn to_message_bytes(&self) -> [u8; 8] {
        let register_code: u16 = self.register.into();
        let mut message_bytes: [u8; 8] = [self.device_address, self.command.into(), (register_code >> 8) as u8, register_code as u8, (self.value >> 8) as u8, self.value as u8, 0x0, 0x0];

        let crc = crc16(&message_bytes[0..6]);
        message_bytes[6] = (crc >> 8) as u8;
        message_bytes[7] = crc as u8;

        message_bytes
    }
}

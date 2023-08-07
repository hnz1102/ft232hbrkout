use rppal::i2c::I2c;
use std::error::Error;
 
const DEVADDR: u16 = 0x50;
 
pub struct AT24EEPROM {
    bus: I2c,
}
 
impl AT24EEPROM {
    pub fn new(bus: I2c) -> Self {
        Self { bus }
    }
 
    pub fn get_data(&mut self, addr: u8) -> Option<u8> {
        self.bus
            .set_slave_address(DEVADDR)
            .expect("failed to set slave address");
        match self.bus.smbus_read_byte(addr) {
            Ok(n) => {
                Some(n)
            },
            Err(e) => {
                eprintln!("{e}");
                None
            }
        }
    }
 
    pub fn write_data(&mut self, addr: u8, value: u8) -> Result<(), Box<dyn Error>>{
        self.bus
            .set_slave_address(DEVADDR)
            .expect("failed to set slave address");
 
        match self.bus.smbus_write_byte(addr, value) {
            Ok(()) => {
                Ok(())
            },
            Err(e) => {
                eprintln!("{e}");
                Err(Box::new(e))
            }
        }
    }
}
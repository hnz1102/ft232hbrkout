use rppal::i2c::I2c;
use std::{error::Error, thread, time::Duration};
 
mod at24mac;
 
fn main() -> Result<(), Box<dyn Error>> {
    let i2c_bus = I2c::new()?;
 
    let mut at24eeprom_dev = at24mac::AT24EEPROM::new(i2c_bus);
 
    let mut value = 255;
 
    for addr in 0..=255 {
        // Write data
        match at24eeprom_dev.write_data(addr, value){
            Ok(()) => {},
            Err(e) => {
                println!("{e}");
            }
        }
        value -= 1;
        thread::sleep(Duration::from_millis(5));
    }

    value = 255;
    for addr in 0..=255 {
        // Read data
        if let Some(ret_value) = at24eeprom_dev.get_data(addr) {
            let mut flag = "NG";
            if value == ret_value {
                flag = "OK";
            }
            println!("addr {addr} value: {value}  {ret_value} {flag}");
        }
        value -= 1;
    } 
    Ok(())
}

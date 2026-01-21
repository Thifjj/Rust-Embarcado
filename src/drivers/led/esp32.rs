use esp_idf_hal::gpio::*;
use crate::drivers::led::hw::LedHw;

pub struct Esp32Led{
    pin: PinDriver<'static, Gpio2, Output>,
}
impl Esp32Led {
    pub fn new(pin: PinDriver<'static, Gpio2, Output>) -> Self {
        Self { pin}
    }
}

impl LedHw for Esp32Led{
    fn set(&mut self, on: bool){
        if on{
            let _ = self.pin.set_high();
        }else{
            let _ = self.pin.set_low();
        }
    }
}
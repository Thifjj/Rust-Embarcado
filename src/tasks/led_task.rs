use esp_idf_hal::delay::FreeRtos;

use crate::drivers::led::fsm::LedFsm;
use crate::drivers::led::hw::LedHw;

const TICK_MS: u32 = 10;

pub fn task_led(mut led: impl LedHw) {
    let mut fsm = LedFsm::new();

    loop {
        if let Some(on) = fsm.on_event(crate::drivers::led::fsm::Event::Tick) {
            led.set(on);
        }

        FreeRtos::delay_ms(TICK_MS);
    }
}

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use std::time::{Instant, Duration};
enum LedState {
    On,
    Off,
    LongOn,
    LongOnBlink,
}

fn main() -> anyhow::Result<()> {

    let peripherals = Peripherals::take().unwrap();
    //pino do led
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let mut state = LedState::Off;
    let mut last_change = Instant::now();
    //como ciclo somente  ate == 10 usei u8
    let mut ciclo : u8 = 0;
    let mut blink_count: u8 = 0;
    let mut led_on = false;


    loop {
        match state{
            LedState::On => {

                    if last_change.elapsed() >= Duration::from_millis(300){
                        led.set_low()?;
                        state = LedState::Off;
                        last_change = Instant::now();
                        ciclo += 1;

                        if ciclo >= 10{
                            blink_count = 0;
                            led_on = false;
                            state = LedState::LongOnBlink;
                        }
                    }
            }

            LedState::Off => {

                if last_change.elapsed() >= Duration::from_millis(700){ 
                        led.set_high()?;
                        last_change = Instant::now();
                        state = LedState::On;
                }
            }
            
            LedState::LongOn => {
                if last_change.elapsed() >= Duration::from_millis(2000){
                        led.set_low()?;
                        last_change = Instant::now();
                        ciclo = 0;
                        state = LedState::Off;
                }
            }

            LedState::LongOnBlink => {
                if last_change.elapsed() >= Duration::from_millis(100){
                    if led_on {
                        led.set_low()?;
                    }else{
                        led.set_high()?;
                    }

                    led_on = !led_on;
                    last_change = Instant::now();

                    if !led_on{
                        blink_count +=1;
                    }

                    if blink_count >= 5 {
                        led.set_high()?;
                        last_change = Instant::now();
                        state = LedState::LongOn;
                    }
                }
            }
        }
        FreeRtos::delay_ms(10);
    }
}

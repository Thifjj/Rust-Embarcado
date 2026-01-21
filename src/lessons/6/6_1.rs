use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use std::time::{Instant, Duration};
use esp_idf_hal::delay::FreeRtos;

fn task_led(led: &mut PinDriver<'_, Gpio2, Output>, last: &mut Instant) -> anyhow::Result<()> {
    if last.elapsed() >= Duration::from_millis(500) {
        led.toggle()?;
        *last = Instant::now();
    }
    Ok(())
}

fn task_log(last: &mut Instant) {
    if last.elapsed() >= Duration::from_secs(2) {
        println!("Sistema OK");
        *last = Instant::now();
    }
}

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let mut led_timer = Instant::now();
    let mut log_timer = Instant::now();

    loop {
        task_led(&mut led, &mut led_timer)?;
        task_log(&mut log_timer);
        FreeRtos::delay_ms(10);
    }
}

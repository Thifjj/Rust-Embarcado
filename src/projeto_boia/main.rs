mod pwm;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use pwm::pwm::ManualPwm;
use esp_idf_hal::delay::FreeRtos;

fn main() {
    const TICK_MS: u32 = 10;
    let peripherals = Peripherals::take().unwrap();

    // Boia -> GPIO4
    let mut boia = PinDriver::input(peripherals.pins.gpio4).unwrap();
    boia.set_pull(Pull::Up).unwrap();

    // LED -> GPIO18
    let led = PinDriver::output(peripherals.pins.gpio18).unwrap();

    // Buzzer -> GPIO19 (ainda sem PWM)
    let buzzer_pin = PinDriver::output(peripherals.pins.gpio19).unwrap();
    let mut buzzer = ManualPwm::new(buzzer_pin,led, 2000, ); // 2 kHz

    println!("Sistema iniciado");
    buzzer.start_c4();

    loop {
        // LIGA
        buzzer.update();
        if buzzer.is_done() {
            FreeRtos::delay_ms(3000);
            buzzer.start_c4();
        }

        FreeRtos::delay_ms(TICK_MS);
    }
}

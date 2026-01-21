mod drivers;
mod tasks;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;

use drivers::led::esp32::Esp32Led;
use drivers::led::fake_placa::FakePlaca;

fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pin = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let led = Esp32Led::new(pin);
    // ou:
    // let led = FakePlaca::new("TESTE");

    led_task::task_led(led);
}

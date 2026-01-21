use esp_idf_hal::gpio::{Output, PinDriver};
use std::{thread, time::Duration};

pub enum C4State {
    Idle,
    Slow,
    Medium,
    Fast,
    Explode,
    Done,
}

pub struct ManualPwm<'a, P, L>
where
    P: esp_idf_hal::gpio::OutputPin,
    L: esp_idf_hal::gpio::OutputPin,
{
    buzzer: PinDriver<'a, P, Output>,
    led: PinDriver<'a, L, Output>,

    high_us: u64,
    low_us: u64,

    state: C4State,
    counter: u8,
}

impl<'a, P, L> ManualPwm<'a, P, L>
where
    P: esp_idf_hal::gpio::OutputPin,
    L: esp_idf_hal::gpio::OutputPin,
{
    pub fn new(
        buzzer: PinDriver<'a, P, Output>,
        led: PinDriver<'a, L, Output>,
        freq_hz: u32,
    ) -> Self {
        let period_us = 1_000_000u64 / freq_hz as u64;
        let half = period_us / 2;

        Self {
            buzzer,
            led,
            high_us: half,
            low_us: half,
            state: C4State::Idle,
            counter: 0,
        }
    }

    fn pwm_tick(&mut self) {
        self.buzzer.set_high().ok();
        thread::sleep(Duration::from_micros(self.high_us));
        self.buzzer.set_low().ok();
        thread::sleep(Duration::from_micros(self.low_us));
    }

    fn beep_with_led(&mut self, ms: u64) {
        self.led.set_high();

        let cycles = (ms * 1000) / (self.high_us + self.low_us);
        for _ in 0..cycles {
            self.pwm_tick();
        }

        self.led.set_low();
    }

    pub fn start_c4(&mut self) {
        self.state = C4State::Slow;
        self.counter = 0;
    }

    pub fn is_done(&self) -> bool {
        self.state == C4State::Done
    }

    pub fn update(&mut self) {
        match self.state {
            C4State::Idle => {}

            C4State::Slow => {
                self.beep_with_led(100);
                thread::sleep(Duration::from_millis(600));
                self.counter += 1;

                if self.counter >= 23 {
                    self.state = C4State::Medium;
                    self.counter = 0;
                }
            }

            C4State::Medium => {
                self.beep_with_led(60);
                thread::sleep(Duration::from_millis(300));
                self.counter += 1;

                if self.counter >= 33 {
                    self.state = C4State::Fast;
                    self.counter = 0;
                }
            }

            C4State::Fast => {
                self.beep_with_led(30);
                thread::sleep(Duration::from_millis(120));
                self.counter += 1;

                if self.counter >= 53 {
                    self.state = C4State::Explode;
                }
            }

            C4State::Explode => {
                self.led.set_high().ok();
                self.beep_with_led(3000); // 💥 4s
                self.led.set_low().ok();
                self.state = C4State::Done;
            }


            C4State::Done => {
                self.buzzer.set_low().ok();
                self.led.set_low().ok();
            }
        }
    }
}

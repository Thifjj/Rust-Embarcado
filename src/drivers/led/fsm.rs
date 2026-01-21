const OFF_T: u32 = 70;
const ON_T: u32 = 30;
const BLINK_T: u32 = 10;
const LONG_T: u32 = 200;

const MAX_CICLOS:u8 = 10;
const MAX_BLINKS:u8 = 5;
pub enum Event{
    Tick,
    ButtonPress,
}

pub enum LedState {
    On,
    Off,
    LongOn,
    LongOnBlink,
}

pub struct LedFsm {
    state: LedState,
    tick_count : u32,
    ciclo: u8,
    blink_count: u8,
    led_on: bool,
}

impl LedFsm {
    pub fn new() -> Self {
        Self {
            state: LedState::Off,
            tick_count: 0,
            ciclo: 0,
            blink_count: 0,
            led_on: false,
        }
    }

    /// Executa um passo da FSM
    /// Retorna:
    /// - Some(true)  -> LED deve ligar
    /// - Some(false) -> LED deve desligar
    /// - None       -> não muda nada

    pub fn on_event(&mut self, event: Event) -> Option<bool> {
        match event {
            Event::Tick => self.on_tick(),
            Event::ButtonPress => None, // futuro
        }
    }

    fn on_tick(&mut self) -> Option<bool>{
        self.tick_count +=1;
        
        match self.state {
            LedState::On => {
                if self.tick_count >= ON_T {
                    self.ciclo += 1;
                    self.tick_count = 0;
                    self.state = LedState::Off;
                    
                    if self.ciclo >= MAX_CICLOS {
                        self.blink_count = 0;
                        self.led_on = false;
                        self.state = LedState::LongOnBlink;
                    }
                    
                    return Some(false);
                }
            }
            
            LedState::Off => {
                if self.tick_count >= OFF_T {
                    self.tick_count = 0;
                    self.state = LedState::On;
                    return Some(true);
                }
            }
            
            LedState::LongOnBlink => {
                if self.tick_count >= BLINK_T {
                    self.led_on = !self.led_on;
                    self.tick_count = 0;
                    
                    if !self.led_on {
                        self.blink_count += 1;
                    }
                    
                    if self.blink_count >= MAX_BLINKS{
                        self.state = LedState::LongOn;
                        self.tick_count = 0;
                        return Some(true);
                    }
                    
                    return Some(self.led_on);
                }
            }
            
            LedState::LongOn => {
                if self.tick_count >= LONG_T {
                    self.ciclo = 0;
                    self.state = LedState::Off;
                    self.tick_count = 0;
                    return Some(false);
                }
            }
        }
        
        None
    }
}

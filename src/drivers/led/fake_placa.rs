use crate::drivers::led::hw::LedHw;

pub struct FakePlaca{
    name: &'static str,
}

impl FakePlaca{
    pub fn new(name: &'static str) -> Self {
        self { name }
    }
}

impl LedHw for FakePlaca{
    fn set(&mut self, on:bool){
        println!(
            "[{}] LED {}",
            self.name,
            if on { "ON" } else { "OFF" }
        );
    }
}
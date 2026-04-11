use clap::ValueEnum;
use rppal::gpio;
use serde::{Deserialize, Serialize};

pub trait InputSource {
    fn init(&mut self) -> Result<(), String>;
    fn read(&mut self) -> Reading;
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ValueEnum)]
pub enum ChoiceOfInputSource {
    #[allow(unused)]
    #[default]
    Gpio,
    Dummy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reading {
    Low,
    High,
}

pub struct GpioInputSource {
    pin_number: u8,
    pin: Option<gpio::InputPin>,
}

impl GpioInputSource {
    pub fn new(pin_number: u8) -> Self {
        Self {
            pin_number,
            pin: None,
        }
    }
}

impl InputSource for GpioInputSource {
    fn init(&mut self) -> Result<(), String> {
        match self.sub_init() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("GPIO error: {e}")),
        }
    }

    fn read(&mut self) -> Reading {
        match self.pin {
            Some(ref pin) => match pin.read() {
                gpio::Level::Low => Reading::Low,
                gpio::Level::High => Reading::High,
            },
            None => {
                eprintln!("Error: GPIO pin not initialized");
                Reading::Low
            }
        }
    }
}

impl GpioInputSource {
    fn sub_init(&mut self) -> Result<(), gpio::Error> {
        let gpio = gpio::Gpio::new()?;
        let pin = gpio.get(self.pin_number)?.into_input_pullup();
        self.pin = Some(pin);
        Ok(())
    }
}

pub struct MockInputSource {
    counter: u32,
}

impl MockInputSource {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl InputSource for MockInputSource {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn read(&mut self) -> Reading {
        self.counter += 1;

        if self.counter % 30 < 15 {
            Reading::Low
        } else {
            Reading::High
        }
    }
}

use clap::ValueEnum;
use rppal::gpio;
use serde::{Deserialize, Serialize};

use crate::settings;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reading {
    Low,
    High,
}

pub trait InputSource {
    fn init(&mut self) -> Result<(), String>;
    fn read(&mut self) -> Reading;
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ChoiceOfInputSource {
    #[default]
    Gpio,
    Dummy,
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
            Ok(()) => Ok(()),
            Err(e) => Err(format!("GPIO error: {e}")),
        }
    }

    fn read(&mut self) -> Reading {
        if let Some(pin) = &self.pin {
            match pin.read() {
                gpio::Level::Low => Reading::Low,
                gpio::Level::High => Reading::High,
            }
        } else {
            eprintln!("Error: GPIO pin not initialized");
            Reading::Low
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

pub struct DummyInputSource {
    counter: u32,
    modulus: u32,
    threshold: u32,
}

impl DummyInputSource {
    pub fn new(modulus: u32, threshold: u32, settings: &settings::Settings) -> Self {
        if settings.monitor.startup_window > (modulus - threshold) * settings.monitor.loop_interval
        {
            eprintln!(
                "Warning: The startup window ({:?}) is longer than the time it takes for the dummy source to transition from low back to high ({} cycles).",
                settings.monitor.startup_window,
                modulus - threshold
            );
        }
        Self {
            counter: 0,
            modulus,
            threshold,
        }
    }
}

impl InputSource for DummyInputSource {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn read(&mut self) -> Reading {
        self.counter += 1;

        if (self.counter % self.modulus) < self.threshold {
            Reading::Low
        } else {
            Reading::High
        }
    }
}

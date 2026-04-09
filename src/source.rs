use rppal::gpio;

pub trait InputSource {
    fn init(&mut self) -> Result<(), gpio::Error>;
    fn read(&mut self) -> gpio::Level;
}

#[derive(Debug)]
pub enum ChoiceOfInputSource {
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
    fn init(&mut self) -> Result<(), gpio::Error> {
        let gpio = gpio::Gpio::new()?;
        let pin = gpio.get(self.pin_number)?.into_input_pullup();
        self.pin = Some(pin);
        Ok(())
    }

    fn read(&mut self) -> gpio::Level {
        self.pin.as_ref().unwrap().read()
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
    fn init(&mut self) -> Result<(), gpio::Error> {
        Ok(())
    }

    fn read(&mut self) -> gpio::Level {
        self.counter += 1;

        if self.counter % 30 < 15 {
            gpio::Level::Low
        } else {
            gpio::Level::High
        }
    }
}

//! Input source definitions and implementations.

use clap::ValueEnum;
use rppal::gpio;

/// Enum representing a reading from an input source, which can be either
/// electrically LOW or HIGH.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reading {
    /// Represents an electrically LOW reading from an input source.
    Low,

    /// Represents an electrically HIGH reading from an input source.
    High,
}

/// Trait representing an input source from which a `Reading` can be read.
pub trait InputSource {
    /// Name of the input source.
    fn name(&self) -> String;

    /// Initializes the input source, performing any necessary setup that
    /// may fail (and is as such not part of a `new` constructor).
    ///
    /// # Returns
    /// - `Ok(())` if initialization succeeded.
    /// - `Err(String)` if initialization failed, with a string describing the error.
    fn init(&mut self) -> Result<(), String>;

    /// Reads a `Reading` from the input source.
    fn read(&mut self) -> Reading;

    /// Perform's a sanity check on the input source's configuration.
    ///
    /// # Returns
    /// - `Ok(())` if the configuration is sane.
    /// - `Err(Vec<String>)` if the configuration contains errors, with a
    ///   vector of strings describing the errors.
    fn sanity_check(&self) -> Result<(), Vec<String>>;
}

/// Enum representing the choice of input source to use for monitoring.
#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChoiceOfInputSource {
    // Don't make this a /// documentation comment or the --help listing wlil be too big.
    // Represents the GPIO input source that reads from a GPIO pin.
    #[default]
    Gpio,

    // As above.
    // Represents the dummy input source which cycles through `Reading` states.
    Dummy,
}

/// Input source implementation that reads from a GPIO pin using the `rppal` crate.
pub struct GpioInputSource {
    /// GPIO pin number to read from.
    pin_number: u8,

    /// `rppal` pin representation.
    pin: Option<gpio::InputPin>,
}

impl GpioInputSource {
    /// Creates a new `GpioInputSource` with the specified GPIO pin number.
    pub fn new(pin_number: u8) -> Self {
        Self {
            pin_number,
            pin: None,
        }
    }
}

impl InputSource for GpioInputSource {
    /// Name of the input source, including the GPIO pin number in the string.
    fn name(&self) -> String {
        format!("GPIO{}", self.pin_number)
    }

    /// Initializes the GPIO input source.
    fn init(&mut self) -> Result<(), String> {
        match self.sub_init() {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Reads a `Reading` from the GPIO pin.
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

    /// Performs a sanity check on the GPIO input source's configuration.
    ///
    /// # Returns
    /// - `Ok(())` if the configuration is sane.
    /// - `Err(Vec<String>)` if the configuration contains errors,
    ///   with a vector of strings describing the errors.
    fn sanity_check(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.pin_number > 40 {
            errors.push(format!("Invalid GPIO pin number: {}", self.pin_number));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl GpioInputSource {
    /// Helper function to perform the actual initialization of the GPIO pin.
    ///
    /// # Returns
    /// - `Ok(())` if initialization succeeded.
    /// - `Err(gpio::Error)` if initialization failed, with the error from the
    ///   `rppal` crate.
    fn sub_init(&mut self) -> Result<(), gpio::Error> {
        let gpio = gpio::Gpio::new()?;
        let pin = gpio.get(self.pin_number)?.into_input_pullup();
        self.pin = Some(pin);
        Ok(())
    }
}

/// Input source implementation that simulates readings by cycling through `Reading` states.
pub struct DummyInputSource {
    /// Counter to keep track of the number of readings taken.
    counter: u32,

    /// Modulus value to determine the cycle length of the readings.
    modulus: u32,

    /// Threshold value to determine the point in the cycle where readings
    /// transition from `Reading::Low` to `Reading::High`.
    threshold: u32,
}

impl DummyInputSource {
    /// Creates a new `DummyInputSource` with the specified modulus and threshold.
    ///
    /// # Parameters
    /// - `modulus`: The modulus value to determine the cycle length of the readings.
    /// - `threshold`: The threshold value to determine the point in the cycle
    ///   where readings transition from `Reading::Low` to `Reading::High`.
    /// - `settings`: The program settings.
    pub fn new(modulus: u32, threshold: u32) -> Self {
        Self {
            counter: 0,
            modulus,
            threshold,
        }
    }
}

impl InputSource for DummyInputSource {
    /// Name of the input source, which is simply "dummy".
    fn name(&self) -> String {
        "dummy".to_string()
    }

    /// Initializes the dummy input source.
    ///
    /// Literally does nothing.
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Produces a "reading" by cycling through `Reading` states.
    fn read(&mut self) -> Reading {
        self.counter += 1;

        if (self.counter % self.modulus) < self.threshold {
            Reading::Low
        } else {
            Reading::High
        }
    }

    /// Performs a sanity check on the dummy input source's configuration.
    ///
    /// # Returns
    /// - `Ok(())` if the configuration is sane.
    /// - `Err(Vec<String>)` if the configuration contains errors,
    ///   with a vector of strings describing the errors.
    fn sanity_check(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.modulus == 0 {
            errors.push("Modulus may not be zero".to_string());
        }

        if self.threshold > self.modulus {
            errors.push("Threshold may not be greater than modulus".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

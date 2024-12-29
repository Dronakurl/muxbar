use battery::{Manager, State};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BatteryError;

impl fmt::Display for BatteryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Battery error")
    }
}

impl Error for BatteryError {}

#[derive(Copy, Clone)]
pub struct BatteryInformation {
    pub percentages: u8,
    pub is_charging: bool,
}

impl BatteryInformation {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new()?;
        let mut batteries = manager.batteries()?;
        let battery = batteries.next().ok_or(BatteryError)??;

        let percentages = battery
            .state_of_charge()
            .get::<battery::units::ratio::percent>() as u8;

        let is_charging = battery.state() != State::Discharging;

        Ok(Self {
            percentages,
            is_charging,
        })
    }
}

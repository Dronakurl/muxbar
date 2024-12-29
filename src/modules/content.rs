use chrono::{DateTime, Local};
use std::error::Error;

pub trait Content {
    fn show(&self) -> Result<String, Box<dyn Error>>;
}

pub struct TimeModule(String);

pub enum Module {
    Time(String),
    Date(String),
}

impl Content for Module {
    fn show(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Module::Time(time) => TimeModule(time.clone()).show(),
            Module::Date(date) => DateModule(date.clone()).show(),
        }
    }
}

impl Content for TimeModule {
    fn show(&self) -> Result<String, Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        Ok(now.format(&self.0).to_string())
    }
}

pub struct DateModule(String);

impl Content for DateModule {
    fn show(&self) -> Result<String, Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        Ok(now.format(&self.0).to_string())
    }
}

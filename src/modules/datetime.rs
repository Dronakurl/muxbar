use crate::modules::Show;
use chrono::{DateTime, Local};
use std::error::Error;

pub struct TimeModule(pub String);
impl Show for TimeModule {
    fn show(&self) -> Result<String, Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        Ok(now.format(&self.0).to_string())
    }
}

pub struct DateModule(pub String);
impl Show for DateModule {
    fn show(&self) -> Result<String, Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        Ok(now.format(&self.0).to_string())
    }
}

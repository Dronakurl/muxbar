pub mod datetime;
pub mod styled;

use datetime::{DateModule, TimeModule};
use std::error::Error;

pub trait Show {
    fn show(&self) -> Result<String, Box<dyn Error>>;
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Content {
    Time(String),
    Date(String),
    Manual(&'static str),
}

impl Show for Content {
    fn show(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Content::Time(time) => TimeModule(time.clone()).show(),
            Content::Date(date) => DateModule(date.clone()).show(),
            Content::Manual(s) => Ok(s.to_string()),
        }
    }
}

// // Those are only constructed in config.rs
// #[allow(dead_code)]
// #[derive(Clone, Copy)]
// pub enum Module {
//     Manual(&'static str),
//     Time(&'static str),
//     Battery,
//     Cpu(usize),
//     Memory(usize),
//     Swap(usize),
//     Uptime,
//     SessionName,
//     WindowName,
//     WindowIndex,
//     PaneIndex,
//     Hostname,
// }

// use crate::utils::strings;
// use crate::utils::system::{battery::BatteryInformation, cpu};
// use chrono::{DateTime, Local};
// use std::time::Duration;
// use sysinfo::{MemoryRefreshKind, RefreshKind, System};

// impl Content {
//     fn display(self) -> Result<String, ()> {
//         match self {
//             Content::Manual(s) => Ok(String::from(s)),
//             Content::Time(format) => {
//                 let now: DateTime<Local> = Local::now();
//
//                 Ok(now.format(format).to_string())
//             }
//             Content::Battery => {
//                 BatteryInformation::new().map(|info| format!("{}%", info.percentages))
//             }
//             Content::SessionName => Ok(String::from("#S")),
//             Content::WindowName => Ok(String::from("#W")),
//             Content::WindowIndex => Ok(String::from("#I")),
//             Content::PaneIndex => Ok(String::from("#P")),
//             Content::Hostname => Ok(String::from("#H")),
//             Content::Cpu(rounding) => Ok(strings::round(cpu::get_total_average(), rounding)),
//             Content::Memory(rounding) => {
//                 let mut sys = System::new_with_specifics(
//                     RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
//                 );
//
//                 sys.refresh_memory();
//
//                 let total_memory = sys.total_memory();
//                 let used_memory = sys.used_memory();
//
//                 let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
//                 Ok(strings::round(memory_usage_percent, rounding))
//             }
//             Content::Uptime => {
//                 let uptime = System::uptime();
//                 let uptime = Duration::from_secs(uptime);
//
//                 Ok(format!("{}", strings::PrettyDuration::new(uptime)))
//             }
//             Content::Swap(rounding) => {
//                 let mut sys = System::new_with_specifics(
//                     RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
//                 );
//
//                 sys.refresh_memory();
//
//                 let total_swap = sys.total_swap();
//                 let used_swap = sys.used_swap();
//
//                 let swap_usage_percent = (used_swap as f64 / total_swap as f64) * 100.0;
//                 Ok(strings::round(swap_usage_percent, rounding))
//             }
//         }
//     }
// }

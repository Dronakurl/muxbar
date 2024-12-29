// use crate::modules::Show;
// use chrono::{DateTime, Local};
// use std::error::Error;
//
//
// pub struct Cpu(pub String);
// impl Show for TimeModule {
// Content::Cpu(rounding) => Ok(strings::round(cpu::get_total_average(), rounding)),
// Content::Memory(rounding) => {
//     let mut sys = System::new_with_specifics(
//         RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
//     );
//
//     sys.refresh_memory();
//
//     let total_memory = sys.total_memory();
//     let used_memory = sys.used_memory();
//
//     let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
//     Ok(strings::round(memory_usage_percent, rounding))
// }
// Content::Uptime => {
//     let uptime = System::uptime();
//     let uptime = Duration::from_secs(uptime);
//
//     Ok(format!("{}", strings::PrettyDuration::new(uptime)))
// }

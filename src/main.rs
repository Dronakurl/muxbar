#![allow(dead_code)]
mod colors;
mod config;
mod icons;
mod modules;
mod utils;

use std::thread;

fn main() {
    let modules = config::get_modules();

    #[cfg(debug_assertions)]
    let instant = std::time::Instant::now();

    let strings: Vec<_> = modules
        .into_iter()
        .map(|styled_mod| thread::spawn(move || styled_mod.to_string()))
        .filter_map(|t| t.join().ok())
        .collect();

    #[cfg(debug_assertions)]
    println!("{:?}", instant.elapsed());

    println!(
        "{}{}{}",
        config::pre_modules(),
        strings.join(config::between_modules()),
        config::post_modules(),
    );
}

use cfg_if::cfg_if;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub mod circuit;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log(log_level: log::Level) {
            console_log::init_with_level(log_level).unwrap();
        }
    } else {
        fn init_log(log_level: log::Level) {
            use simplelog::*;
            TermLogger::init(log_level.to_level_filter(), Config::default(), TerminalMode::Stdout).unwrap();
        }
    }
}

#[wasm_bindgen]
pub fn initialize(log_level: &str) {
    match log::Level::from_str(log_level) {
        Ok(level) => init_log(level),
        Err(_) => {
            init_log(log::Level::Debug);
            log::error!("Invalid log level {}", log_level);
        }
    }
}

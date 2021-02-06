use cfg_if::cfg_if;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub mod circuit;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log(log_level: log::Level) {
            console_log::init_with_level(log_leve).expect("error initializing log");
        }
    } else {
        fn init_log(log_level: log::Level) {
            use simplelog::*;
            TermLogger::init(log_level.to_level_filter(), Config::default(), TerminalMode::Stdout).expect("error initializing log");
        }
    }
}

#[wasm_bindgen]
pub fn initialize(log_level: &str) {
    init_log(log::Level::from_str(log_level).expect("error initializing log: invalid log level"));
}

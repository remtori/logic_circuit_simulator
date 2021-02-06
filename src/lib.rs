use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

pub mod circuit;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {
            use simplelog::*;
            TermLogger::init(LevelFilter::Trace, Config::default(), simplelog::TerminalMode::Stdout).expect("error initializing log");
        }
    }
}

#[wasm_bindgen]
pub fn initialize() {
    init_log();
}

use core::console::{ConsoleHandler, CustomConsoleFormatter, CustomLogger};
use std::sync::Arc;

use log::{error, warn, info, debug, LevelFilter};

mod api;
mod data;
mod core;

fn main() {
    let logger = CustomLogger::new()
    .add_handler(Arc::new(ConsoleHandler::new(
        Arc::new(CustomConsoleFormatter::new("[{timestamp}] [{level}] {message}".to_string()))
    )));
    
    log::set_logger(Box::leak(Box::new(logger))).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Debug);
    
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
}


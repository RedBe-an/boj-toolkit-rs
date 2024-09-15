#![allow(dead_code)]

use log::{Level, Metadata, Record, };
use chrono::Local;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;

/// Represents a log entry with timestamp, level, and message.
#[derive(Debug, Clone)]
pub struct LogEntry {
    timestamp: String,
    level: Level,
    message: String,
}

impl LogEntry {
    /// Creates a new LogEntry instance.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level of the entry.
    /// * `message` - The message content of the log entry.
    ///
    /// # Returns
    ///
    /// A new `LogEntry` instance.
    fn new(level: Level, message: &str) -> Self {
        Self {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level,
            message: message.to_string(),
        }
    }
}

/// Defines the interface for log handlers.
pub trait LogHandler: Send + Sync {
    /// Handles a log entry.
    ///
    /// # Arguments
    ///
    /// * `entry` - The log entry to be handled.
    fn handle(&self, entry: &LogEntry);
}

/// Defines the interface for log formatters.
pub trait LogFormatter: Send + Sync {
    /// Formats a log entry into a string.
    ///
    /// # Arguments
    ///
    /// * `entry` - The log entry to be formatted.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the log entry.
    fn format(&self, entry: &LogEntry) -> String;
}

/// Custom logger implementing the Builder pattern.
pub struct CustomLogger {
    handlers: Vec<Arc<dyn LogHandler>>,
}

impl CustomLogger {
    /// Creates a new CustomLogger instance.
    ///
    /// # Returns
    ///
    /// A new `CustomLogger` instance with no handlers.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Adds a new handler to the logger.
    ///
    /// # Arguments
    ///
    /// * `handler` - The log handler to be added.
    ///
    /// # Returns
    ///
    /// The `CustomLogger` instance for method chaining.
    pub fn add_handler(mut self, handler: Arc<dyn LogHandler>) -> Self {
        self.handlers.push(handler);
        self
    }
}

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_entry = LogEntry::new(record.level(), &record.args().to_string());
            for handler in &self.handlers {
                handler.handle(&log_entry);
            }
        }
    }

    fn flush(&self) {}
}

/// Console handler for outputting logs to the console.
pub struct ConsoleHandler {
    formatter: Arc<dyn LogFormatter>,
}

impl ConsoleHandler {
    /// Creates a new ConsoleHandler instance.
    ///
    /// # Arguments
    ///
    /// * `formatter` - The formatter to use for formatting log entries.
    ///
    /// # Returns
    ///
    /// A new `ConsoleHandler` instance.
    pub fn new(formatter: Arc<dyn LogFormatter>) -> Self {
        Self { formatter }
    }
}

impl LogHandler for ConsoleHandler {
    fn handle(&self, entry: &LogEntry) {
        println!("{}", self.formatter.format(entry));
    }
}

/// Custom console formatter for flexible log formatting.
pub struct CustomConsoleFormatter {
    format_string: String,
}

impl CustomConsoleFormatter {
    /// Creates a new CustomConsoleFormatter instance.
    ///
    /// # Arguments
    ///
    /// * `format_string` - The format string to use for formatting log entries.
    ///
    /// # Returns
    ///
    /// A new `CustomConsoleFormatter` instance.
    pub fn new(format_string: String) -> Self {
        Self { format_string }
    }
}

impl LogFormatter for CustomConsoleFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        let mut result = self.format_string.clone();
        result = result.replace("{timestamp}", &entry.timestamp);
        result = result.replace("{level}", &entry.level.to_string());
        result = result.replace("{message}", &entry.message);
        
        result = result.replace("{level}", &entry.level.to_string().color(match entry.level {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Green,
            Level::Debug => Color::Blue,
            Level::Trace => Color::Magenta,
        }).to_string());
        
        result
    }
}

/// File handler for writing logs to a file.
pub struct FileHandler {
    file_path: String,
    formatter: Arc<dyn LogFormatter>,
}

impl FileHandler {
    /// Creates a new FileHandler instance.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the log file.
    /// * `formatter` - The formatter to use for formatting log entries.
    ///
    /// # Returns
    ///
    /// A new `FileHandler` instance.
    #[allow(dead_code)]
    pub fn new(file_path: &str, formatter: Arc<dyn LogFormatter>) -> Self {
        Self {
            file_path: file_path.to_string(),
            formatter,
        }
    }
}

impl LogHandler for FileHandler {
    fn handle(&self, entry: &LogEntry) {
        let formatted = self.formatter.format(entry);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .expect("Failed to open log file");

        file.write_all(formatted.as_bytes())
            .expect("Failed to write log");
    }
}

/// Custom file formatter for flexible log file formatting.
pub struct CustomFileFormatter {
    format_string: String,
}

impl CustomFileFormatter {
    /// Creates a new CustomFileFormatter instance.
    ///
    /// # Arguments
    ///
    /// * `format_string` - The format string to use for formatting log entries.
    ///
    /// # Returns
    ///
    /// A new `CustomFileFormatter` instance.
    #[allow(dead_code)]
    pub fn new(format_string: String) -> Self {
        Self { format_string }
    }
}

impl LogFormatter for CustomFileFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        let mut result = self.format_string.clone();
        result = result.replace("{timestamp}", &entry.timestamp);
        result = result.replace("{level}", &entry.level.to_string());
        result = result.replace("{message}", &entry.message);
        result + "\n"
    }
}

// Example usage:
//
// fn main() {
//     init_logger().expect("Failed to initialize logger");
//
//     log::error!("This is an error message");
//     log::warn!("This is a warning message");
//     log::info!("This is an info message");
//     log::debug!("This is a debug message");
// }
//
// Custom formatter example:
//
// let custom_format = "{timestamp} | {level} | {message}".to_string();
// let custom_formatter = Arc::new(CustomConsoleFormatter::new(custom_format));
// let custom_handler = Arc::new(ConsoleHandler::new(custom_formatter));
// let logger = CustomLogger::new().add_handler(custom_handler);
// log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
//
// After setting up the logger, you can use the log macros throughout your code:
//
// log::info!("Application started");
// log::warn!("Unusual behavior detected");
// log::error!("Critical error occurred: {}", error_message);
// log::debug!("Debug information: {:?}", debug_data);





















#![allow(dead_code)]

pub enum LogLevel {
    Info,
    Error,
    Warning,
    Debug,
}

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info, warn};
use std::io;

pub struct ConsoleController {
    term: Term,
}

impl ConsoleController {
    pub fn new() -> Self {
        ConsoleController {
            term: Term::stdout(),
        }
    }
    pub fn info<T: std::fmt::Display>(&self, message: T) {
        println!("{} {}", style("[INFO]").cyan(), message);
        info!("{}", message);
    }

    pub fn error<T: std::fmt::Display>(&self, message: T) {
        eprintln!("{} {}", style("[ERROR]").red().bold(), message);
        error!("{}", message);
    }
    
    pub fn warning<T: std::fmt::Display>(&self, message: T) {
        println!("{} {}", style("[WARNING]").yellow(), message);
        warn!("{}", message);
    }
    
    pub fn debug<T: std::fmt::Display>(&self, message: T) {
        println!("{} {}", style("[DEBUG]").magenta(), message);
        debug!("{}", message);
    }
    

    pub fn ask_input(&self, prompt: &str) -> io::Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn ask_confirm(&self, prompt: &str) -> io::Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn ask_select<T: ToString>(&self, prompt: &str, items: &[T]) -> io::Result<usize> {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(items)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn show_progress(&self, total: u64) -> ProgressBar {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.blue} [{elapsed_precise}] [{bar:50.blue/cyan}] {pos}/{len} {eta}",
                )
                .unwrap()
                .progress_chars("▰▱"),
        );
        pb
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        self.term.clear_screen()
    }

    pub fn set_title(&self, title: &str) {
        self.term.set_title(title);
    }

    pub fn ask_password(&self, prompt: &str, confirm: &str, mismatch: &str) -> String {
        let password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .with_confirmation(confirm, mismatch)
            .interact()
            .unwrap();

        password
    }

    pub fn print_table(&self, headers: &[&str], rows: &[Vec<String>]) {
        let max_widths: Vec<usize> = headers
            .iter()
            .enumerate()
            .map(|(i, &h)| {
                rows.iter()
                    .map(|r| r[i].len())
                    .chain(std::iter::once(h.len()))
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        let print_row = |row: &[String]| {
            for (i, cell) in row.iter().enumerate() {
                print!("| {:<width$} ", cell, width = max_widths[i]);
            }
            println!("|");
        };

        let separator = max_widths
            .iter()
            .map(|&w| "-".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("+");

        println!("+{}+", separator);
        print_row(&headers.iter().map(|&s| s.to_string()).collect::<Vec<_>>());
        println!("+{}+", separator);

        for row in rows {
            print_row(row);
        }

        println!("+{}+", separator);
    }
}

mod api;
mod data;
mod core;
mod cmd;

use core::console::ConsoleController;

fn main() {
    let console = ConsoleController::new();
    let password = console.ask_password(
        "Enter your password", 
        "Confirm password", 
        "Passwords mismatching");
    console.info(format!("Your password is: {password}"));
}
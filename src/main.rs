use std::process::exit;

use inquire::{InquireError, Select};

fn main() {
    loop {
        let options: Vec<&str> = vec![
            "1. Create Group",
            "2. Add Expense",
            "3. Split Bill",
            "4. Exit",
        ];

        let selected_prompt: Result<&str, InquireError> =
            Select::new("Please Select one Option", options).prompt();

        match selected_prompt {
            Ok(choice) => {
                handle_choice(choice);
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }
}

fn handle_choice(choice: &str) {
    match choice {
        "1. Create Group" => create_group(),
        "2. Add Expense" => add_expense(),
        "3. Split Bill" => split_bill(),
        "4. Exit" => handle_exit(),
        _ => println!("Invalid choice!"),
    }
}

fn create_group() {
    println!("Creating Group...");
}

fn add_expense() {
    println!("Adding Expense...");
}

fn split_bill() {
    println!("Splitting Bill...");
}
fn handle_exit() {
    println!("Exiting CLI SUI");
    exit(0);
}

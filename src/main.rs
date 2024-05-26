use std::process::exit;
mod split;
use inquire::{InquireError, Select, Text};
use split::*;

fn main() {
    let mut all_users: Vec<User> = Vec::new();
    let mut all_transactions = Transactions::new();

    loop {
        let options = vec![
            "1. Create Group",
            "2. Add Expense",
            "3. Split Bill",
            "4. View all Transactions",
            "5. Exit",
        ];

        let selected_prompt: Result<&str, InquireError> =
            Select::new("Please select one option", options).prompt();

        match selected_prompt {
            Ok(choice) => handle_choice(choice, &mut all_users, &mut all_transactions),
            Err(_) => println!("There was an error, please try again"),
        }
        println!();
    }
}

fn handle_choice(choice: &str, all_users: &mut Vec<User>, all_transactions: &mut Transactions) {
    match choice {
        "1. Create Group" => create_group(all_users),
        "2. Add Expense" => add_expense(all_users, all_transactions),
        "3. Split Bill" => split_bill(all_transactions),
        "4. View all Transactions" => view_all_expenses(all_transactions),
        "5. Exit" => handle_exit(),
        _ => println!("Invalid choice!"),
    }
}

fn create_group(all_users: &mut Vec<User>) {
    let no_of_people = Text::new("Enter the number of people:").prompt();
    match no_of_people {
        Ok(no_of_people) => {
            let mut no_of_people = no_of_people.parse::<u8>().unwrap();
            if no_of_people > 0 {
                while no_of_people > 0 {
                    let username = Text::new("Enter the user name:").prompt();
                    match username {
                        Ok(username) => {
                            let user = User::create_user(&username);
                            all_users.push(user);
                        }
                        Err(_) => println!("Error in creating user"),
                    }
                    no_of_people -= 1;
                }
            }
        }
        Err(_) => println!("Error occurred while taking number of people"),
    }
    println!("------------------Group Created----------------------");
}

fn add_expense(all_users: &mut Vec<User>, all_transactions: &mut Transactions) {
    let mut payer_list: Vec<&str> = Vec::new();
    for user in all_users.iter() {
        payer_list.push(&user.name);
    }

    let selected_user: Result<&str, InquireError> =
        Select::new("Please select the payer of expense:", payer_list.clone()).prompt();

    match selected_user {
        Ok(selected_user) => {
            let giver_list: Vec<&str> = payer_list
                .into_iter()
                .filter(|&name| name != selected_user)
                .collect();

            let giver_name: Result<&str, InquireError> =
                Select::new("Please select the givers of expense:", giver_list.clone()).prompt();

            match giver_name {
                Ok(giver_name) => {
                    let amount = Text::new("Enter the amount of expense:").prompt();
                    match amount {
                        Ok(amount) => {
                            let amount = amount.parse::<u64>().unwrap();
                            let from = User::create_user(selected_user);
                            let to = User::create_user(giver_name);

                            let tx = Transaction::new(from, to, amount);
                            all_transactions.add(tx);
                        }
                        Err(_) => println!("Error in entering amount"),
                    }
                }
                Err(_) => println!("Error in selecting giver"),
            }
        }
        Err(_) => println!("Error in selecting payer"),
    }
    println!("Added Expense...");
}

fn split_bill(all_transactions: &mut Transactions) {
    println!();
    println!("Split Wise Bill ");
    let selected_tx = all_transactions.split_bill();
    selected_tx.display();
}
fn view_all_expenses(all_transactions: &mut Transactions) {
    all_transactions.display();
}

fn handle_exit() {
    println!("Exiting CLI...");
    exit(0);
}

use std::process::exit;
mod split;
use inquire::{InquireError, Select, Text};
use split::*;

fn main() {
    let mut all_users: Vec<User> = Vec::new();
    let mut all_transaction = Transactions::new();

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
                handle_choice(choice, &mut all_users, &mut all_transaction);
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }
}

fn handle_choice(choice: &str, all_users: &mut Vec<User>, all_transaction: &mut Transactions) {
    match choice {
        "1. Create Group" => create_group(all_users),
        "2. Add Expense" => add_expense(all_users, all_transaction),
        "3. Split Bill" => split_bill(all_transaction),
        "4. Exit" => handle_exit(),
        _ => println!("Invalid choice!"),
    }
}

fn create_group(all_users: &mut Vec<User>) {
    let no_of_people = Text::new("Enter the number of people ?").prompt();
    match no_of_people {
        Ok(no_of_people) => {
            let mut no_of_people = no_of_people.parse::<u8>().unwrap();
            if no_of_people > 0 {
                while no_of_people > 0 {
                    let username = Text::new("Enter the User Name ").prompt();
                    match username {
                        Ok(username) => {
                            let user = User::create_user(&username);
                            all_users.push(user);
                        }
                        Err(_) => println!("Error in create user"),
                    }
                    no_of_people -= 1;
                }
            }
        }
        Err(_) => println!("Error occured while taking no of peopel"),
    }
    println!("------------------Group Created----------------------");
}

fn add_expense(all_users: &mut Vec<User>, all_transaction: &mut Transactions) {
    let mut payer_list: Vec<&str> = Vec::new();
    for user in all_users.iter() {
        payer_list.push(&user.name);
    }

    let selected_user: Result<&str, InquireError> =
        Select::new("Please select the payer of Expense ?", payer_list.clone()).prompt();

    match selected_user {
        Ok(selected_user) => {
            let giver_list: Vec<&str> = payer_list
                .into_iter()
                .filter(|&name| name != selected_user)
                .collect();

            let giver_name: Result<&str, InquireError> =
                Select::new("Please select the givers of Expense", giver_list.clone()).prompt();
            match giver_name {
                Ok(giver_name) => {
                    let amount = Text::new("Enter the Amount of Expense").prompt();
                    match amount {
                        Ok(amount) => {
                            let amount = amount.parse::<u64>().unwrap();
                            let from = User::create_user(selected_user);
                            let to = User::create_user(giver_name);

                            let tx = Transaction::new(from, to, amount);
                            all_transaction.add(tx);
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    println!("Added Expense...");
}

fn split_bill(all_transaction: &mut Transactions) {
    println!("Splitting Bill...");
    let seletted_tx = all_transaction.split_bill();
    seletted_tx.display();
}
fn handle_exit() {
    println!("Exiting CLI SUI");
    exit(0);
}

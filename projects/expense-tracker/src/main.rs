mod commands;
mod models;
mod store;

use std::io::{self, Write};
use crate::commands::{add_expense, list_expenses, modify_expense, remove_expense};
use crate::store::{save_to_file, load_from_file};
use crate::models::Expense;

fn main() {
    let mut expenses = load_from_file().unwrap_or_else(|_| Vec::new());

    loop {
        println!("\n--- Expense Tracker ---");
        println!("1. Add Expense");
        println!("2. List Expenses");
        println!("3. Modify Expense");
        println!("4. Remove Expense");
        println!("5. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => add_expense(&mut expenses),
            "2" => list_expenses(&expenses),
            "3" => modify_expense(&mut expenses),
            "4" => remove_expense(&mut expenses),
            "5" => {
                save_to_file(&expenses).unwrap();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

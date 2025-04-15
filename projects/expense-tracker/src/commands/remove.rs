use std::io;
use crate::models::Expense;

pub fn remove_expense(expenses: &mut Vec<Expense>) {
    println!("Enter the ID of the expense to remove: ");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).unwrap();


    let id: u32 = id_input.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input, please enter a valid ID.");
        0
    });

    if let Some(pos) = expenses.iter().position(|expense| expense.id == id) {
        expenses.remove(pos);
        println!("✔ Expense with ID {} has been removed!", id);
    } else {
        println!("❌ Expense with ID {} not found.", id);
    }
}

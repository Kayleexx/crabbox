use std::io;
use crate::models::Expense;

pub fn add_expense(expenses: &mut Vec<Expense>) {
    println!("Enter description: ");
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();

    println!("Enter amount: ");
    let mut amt = String::new();
    io::stdin().read_line(&mut amt).unwrap();

    println!("Enter date (YYYY-MM-DD): ");
    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();

    let amount: f64 = amt.trim().parse().unwrap_or(0.0);

    // Get the next ID based on the last entry
    let id = expenses.last().map(|e| e.id + 1).unwrap_or(1);

    let expense = Expense {
        id,
        description: desc.trim().to_string(),
        amount,
        date: date.trim().to_string(),
    };

   
    expenses.push(expense);

    println!("✔ Expense added!");
}

use std::io;
use crate::models::Expense;

pub fn modify_expense(expenses: &mut Vec<Expense>) {
    println!("Enter the ID of the expense to modify: ");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).unwrap();  

    let id: u32 = match id_input.trim().parse() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            println!("Invalid input, please enter a valid ID.");
            return; // Return early if invalid input
        }
    };

    // Search for the expense with the given ID
    if let Some(expense) = expenses.iter_mut().find(|e| e.id == id) {

        println!("Enter new description (or press enter to keep current): ");
        let mut desc = String::new();
        io::stdin().read_line(&mut desc).unwrap();
        
        println!("Enter new amount (or press enter to keep current): ");
        let mut amt = String::new();
        io::stdin().read_line(&mut amt).unwrap();

        println!("Enter new date (YYYY-MM-DD) (or press enter to keep current): ");
        let mut date = String::new();
        io::stdin().read_line(&mut date).unwrap();

        // Modify fields if input is provided
        if !desc.trim().is_empty() {
            expense.description = desc.trim().to_string();
        }

        if let Ok(new_amount) = amt.trim().parse::<f64>() {
            expense.amount = new_amount;
        }

        if !date.trim().is_empty() {
            expense.date = date.trim().to_string();
        }

        println!("✔ Expense with ID {} has been updated!", id);
    } else {
        println!("❌ Expense with ID {} not found.", id);  // If the expense wasn't found
    }
}

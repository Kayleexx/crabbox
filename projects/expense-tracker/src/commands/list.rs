use crate::models::Expense;

pub fn list_expenses(expenses: &Vec<Expense>) {
    if expenses.is_empty() {
        println!("No expenses to display.");
        return;
    }

    for expense in expenses {
        println!("ID: {}, Desc: {}, Amount: {}, Date: {}", 
            expense.id, expense.description, expense.amount, expense.date);
    }
}

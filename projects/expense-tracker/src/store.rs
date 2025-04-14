use std::fs::File;
use std::io::{self, Write, Read};
use serde_json;
use crate::models::Expense;

pub fn save_to_file(expenses: &Vec<Expense>) -> io::Result<()> {
    let json_data = serde_json::to_string(expenses)?;
    let mut file = File::create("expenses.json")?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_from_file() -> io::Result<Vec<Expense>> {
    let mut file = File::open("expenses.json")?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    let expenses: Vec<Expense> = serde_json::from_str(&json_data)?;
    Ok(expenses)
}

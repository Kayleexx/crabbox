use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]

pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: f64,
    pub date: String,
}
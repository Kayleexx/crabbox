pub mod add;
pub mod list;
pub mod remove;
pub mod modify_expense; // Correctly reference the modify_expense module

pub use add::add_expense;
pub use list::list_expenses;
pub use remove::remove_expense;
pub use modify_expense::modify_expense; // Correctly import modify_expense
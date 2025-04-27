# 🦀 crabbox

Welcome to **crabbox**, a collection of bite-sized Rust projects 🧠 aimed at sharpening my Rust skills through real-world use cases.

Each subproject focuses on a specific concept or domain—like file handling, CLI interactivity, or data modeling—making it a great learning and exploration space for both me and anyone interested in practical Rust.

---

## 📦 Projects

### ✅ 1. CLI Expense Tracker
> A command-line based expense tracking tool with structured commands.

📁 `src/commands`:  
Includes modular commands:
- `add`: Add a new expense
- `list`: List all expenses
- `remove`: Remove an expense
- `mod`: Modify an existing record

🧠 Features:
- Reads and writes expenses to a local JSON file
- Follows idiomatic Rust project structure
- Error handling with `Result` and `unwrap_or_else`
- Uses `Vec<Expense>` in-memory, synced on app exit

---

### ✅ 2. CLI Todo App
> A Todo manager that reads/writes to a local JSON file.

Planned features:
- Add, list, update, and delete tasks
- Mark tasks as done/undone
- Due dates and categories
- Neat CLI prompts using `dialoguer` or `clap`

---

### ✅ 3. Notes CRUD API (In-memory storage)
> A simple Notes API built with Rust, using Actix-web and in-memory storage for quick testing.

🧠 Features:
- Create, read, update, and delete notes
- REST API structure with Actix-web
- In-memory storage using a `HashMap`
- Simple error handling and response formatting

---

### ✅ 4. Book Library API (Actix-web + Diesel ORM + PostgreSQL)
> A REST API for managing books, using Actix-web, Diesel ORM, and PostgreSQL.

🧠 Features:
- Add, get, and list books in the library
- PostgreSQL database integration with Diesel ORM
- Routes for managing book data
- Connection pooling with `diesel::r2d2::ConnectionManager`
- Struct mapping with Diesel schema

---

## 🚧 More coming soon

---

## 🧪 Goals

- Practice idiomatic Rust (ownership, lifetimes, modules)
- Learn structured error handling with `thiserror`
- Get comfortable with serialization (`serde`)
- Build confidence through repetition

---

## 🧑‍💻 Author

Built with ❤️ and curiosity by [Mitali].

---

## 🦀 Why Rust?

Because it forces me to think clearly, write safely, and rage respectfully.


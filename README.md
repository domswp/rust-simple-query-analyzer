# 🧠 Simple Query Analyzer

> A minimal CLI tool written in Rust to analyze basic SQL-like queries (SELECT, INSERT, UPDATE). Ideal for learning parsing and modular design in Rust projects.

---

## ✨ Features

- ✅ Parse simple SQL queries (SELECT, INSERT, UPDATE)
- ✅ Extract query type, table name, columns, and conditions
- ✅ Modular code with clear separation: `parser`, `analyzer`, and `types`
- ✅ Clean terminal output
- 🛠️ Easily extensible to support DELETE, CREATE, etc

---

## 📦 Example Input

```rust
let queries = vec![
    "SELECT name, age FROM users WHERE age > 18",
    "INSERT INTO users (name, age) VALUES ('Alice', 30)",
    "UPDATE users SET age = 31 WHERE id = 1",
```

## 🖥️ Example Output

```bash
Query Type: SELECT
Table: users
Columns: ["name", "age"]
Condition: age > 18
--------------------------
Query Type: INSERT
Table: users
Columns: ["name", "age"]
--------------------------
Query Type: UPDATE
Table: users
Condition: id = 1
--------------------------
```

## 🔧 Project Structure

```
src/
├── main.rs         # Entry point
├── parser.rs       # Parses raw query strings into structured data
├── analyzer.rs     # Analyzes and prints parsed queries
└── types.rs        # Shared struct definition (ParsedQuery)
```

## 🚀 Getting Started

```
git clone https://github.com/yourusername/simple_query_analyzer.git
cd simple_query_analyzer
cargo run
```
Requies rust stable edition


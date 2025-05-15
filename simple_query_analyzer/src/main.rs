mod parser;
mod analyzer;

use parser::parse_query;
use analyzer::analyze_query;

fn main() {
    let queries = vec![
        "SELECT name, age FROM users WHERE age > 18",
        "INSERT INTO users (name, age) VALUES ('Alice', 25)",
        "UPDATE users SET name = 'Bob' WHERE id = 1",
    ];

    for query in queries {
        let parsed = parse_query(query);
        let analysis = analyze_query(&parsed);
        analysis.print_summary();
        println!("------------------------");
    }
}


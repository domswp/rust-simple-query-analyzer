pub struct ParsedQuery {
    pub query_type: String,
    pub table_name: Option<String>,
    pub columns: Vec<String>,
    pub condition: Option<String>,
}

impl ParsedQuery {
    pub fn empty(reason: &str) -> Self {
        println!("Parse failed: {}", reason);
        ParsedQuery {
            query_type: "INVALID".into(),
            table_name: None,
            columns: vec![],
            condition: None,
        }
    }
}

pub fn parse_query(query: &str) -> ParsedQuery {
    let tokens: Vec<&str> = query.split_whitespace().collect();
    if tokens.is_empty() {
        return ParsedQuery::empty("Empty query");
    }

    match tokens[0].to_uppercase().as_str() {
        "SELECT" => parse_select(&tokens),
        "INSERT" => parse_insert(&tokens),
        "UPDATE" => parse_update(&tokens),
        _ => ParsedQuery::empty("Unsupported query type"),
    }
}

fn parse_select(tokens: &[&str]) -> ParsedQuery {
    let mut columns = vec![];
    let mut table = None;
    let mut condition = None;

    if let Some(from_index) = tokens.iter().position(|&t| t.to_uppercase() == "FROM") {
        columns = tokens[1..from_index]
            .join("")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        if from_index + 1 < tokens.len() {
            table = Some(tokens[from_index + 1].to_string());
        }

        if let Some(where_index) = tokens.iter().position(|&t| t.to_uppercase() == "WHERE") {
            condition = Some(tokens[where_index + 1..].join(" "));
        }
    }

    ParsedQuery {
        query_type: "SELECT".into(),
        table_name: table,
        columns,
        condition,
    }
}

fn parse_insert(tokens: &[&str]) -> ParsedQuery {
    let mut table = None;
    let mut columns = vec![];

    if tokens.len() > 2 && tokens[1].to_uppercase() == "INTO" {
        table = Some(tokens[2].to_string());
        if let Some(start) = tokens.iter().position(|&t| t.starts_with('(')) {
            if let Some(end) = tokens.iter().position(|&t| t.contains(')')) {
                columns = tokens[start..=end]
                    .join(" ")
                    .replace('(', "")
                    .replace(')', "")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }
    }

    ParsedQuery {
        query_type: "INSERT".into(),
        table_name: table,
        columns,
        condition: None,
    }
}

fn parse_update(tokens: &[&str]) -> ParsedQuery {
    let table = Some(tokens[1].to_string());
    let condition = if let Some(where_index) = tokens.iter().position(|&t| t.to_uppercase() == "WHERE") {
        Some(tokens[where_index + 1..].join(" "))
    } else {
        None
    };

    ParsedQuery {
        query_type: "UPDATE".into(),
        table_name: table,
        columns: vec![],
        condition,
    }
}


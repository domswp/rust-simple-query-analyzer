use crate::parser::ParsedQuery;

pub struct AnalysisResult<'a> {
    pub parsed: &'a ParsedQuery,
}

impl<'a> AnalysisResult<'a> {
    pub fn print_summary(&self) {
        println!("Query Type: {}", self.parsed.query_type);
        if let Some(table) = &self.parsed.table_name {
            println!("Table: {}", table);
        }
        if !self.parsed.columns.is_empty() {
            println!("Columns: {:?}", self.parsed.columns);
        }
        if let Some(cond) = &self.parsed.condition {
            println!("Condition: {}", cond);
        }
    }
}

pub fn analyze_query(parsed: &ParsedQuery) -> AnalysisResult {
    AnalysisResult { parsed }
}



extern crate rusqlite;

use rusqlite::{Connection, Result};
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

pub fn general_query(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("database.db")?;
    let mut results = Vec::new();

    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        // Assuming the query returns two columns for simplicity
        let col1: String = row.get(0)?;
        let col2: String = row.get(1)?;
        Ok((col1, col2))
    })?;

    for row in rows {
        results.push(row?);
    }

    log_query(query, &results)?;

    Ok(())
}

fn log_query(query: &str, results: &Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().append(true).open(LOG_FILE)?;
    writeln!(file, "```sql\n{}\n```", query)?;
    writeln!(file, "```response from databricks")?;
    for (col1, col2) in results {
        writeln!(file, "{}, {}", col1, col2)?;
    }
    writeln!(file, "```")?;
    Ok(())
}

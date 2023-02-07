mod database;
mod hash;

use anyhow::{Context, Ok, Result};
use clap::Parser;

use crate::database::Database;

#[derive(Parser)]
struct Cli {
    /// Database operation
    operation: String,
    /// The query to execute
    query: String,
}

#[derive(Debug)]
enum Operation {
    Insert,
    Delete,
    SelectAll,
    Get,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // map the operation to an enum
    let operation = match args.operation.as_str() {
        "insert" => Operation::Insert,
        "delete" => Operation::Delete,
        "select_all" => Operation::SelectAll,
        "get" => Operation::Get,
        _ => panic!("Operation not supported"),
    };

    // create a new database
    let db = Database::new("data.csv".to_string());

    println!("Operation: {:?}", operation);
    println!("Query: {}", args.query);

    Ok(())
}

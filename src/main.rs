use anyhow::{Context, Ok, Result};
use clap::Parser;

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

    println!("Operation: {:?}", operation);
    println!("Query: {}", args.query);

    Ok(())
}

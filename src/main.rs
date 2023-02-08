mod database;
mod hash;

use anyhow::{Context, Ok, Result};
use clap::Parser;

use crate::database::Database;
use crate::hash::HashMap;

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

    // test hashmap
    let mut map = HashMap::new();

    map.insert("key".to_string(), "value".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let elem1 = map.get("key".to_string());
    let elem2 = map.get("key2".to_string());

    // print
    println!("elem1: {:?}", elem1.unwrap().value);
    println!("elem2: {:?}", elem2.unwrap().value);

    map.delete("key2".to_string());

    let elem2 = map.get("key2".to_string());
    if elem2.is_none() {
        println!("elem2 is none");
    }

    println!("Operation: {:?}", operation);
    println!("Query: {}", args.query);

    Ok(())
}

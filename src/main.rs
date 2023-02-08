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

    println!("Operation: {:?}", operation);
    println!("Query: {}", args.query);

    Ok(())
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut db = Database::new("data.csv".to_string());

        // insert a new row
        db.insert(vec![
            "John".to_string(),
            "Doe".to_string(),
            "john.doe@example.com".to_string(),
        ]);

        // check that the row was inserted
        assert_eq!(db.get_column(1, "first_name".to_string()).unwrap(), "John");
        assert_eq!(db.get_column(1, "last_name".to_string()).unwrap(), "Doe");
        assert_eq!(
            db.get_column(1, "email".to_string()).unwrap(),
            "john.doe@example.com"
        );

        // check that the next id is correct
        assert_eq!(db.next_id, 2);

        // delete the row
        db.delete(1);

        // check that the row was deleted
        assert_eq!(db.get_column(1, "first_name".to_string()), None);
    }
}

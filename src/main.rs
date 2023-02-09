mod database;
mod hash;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::database::Database;

const DB_PATH: &str = "data.csv";

#[derive(Parser)]
struct Cli {
    /// Database operation (insert, delete, select_all, get)
    operation: String,
    /// The query to execute (depends on the operation)
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
    let mut db = Database::new(DB_PATH.to_string());

    // execute the operation
    match operation {
        Operation::Insert => {
            // get the values from the query
            let values: Vec<&str> = args.query.split(",").collect();
            let values: Vec<String> = values.iter().map(|s| s.to_string()).collect();

            // insert the values into the database
            db.insert(values);
        }
        Operation::Delete => {
            let id = args.query.parse::<u32>().unwrap();
            db.delete(id);
        }
        Operation::SelectAll => {
            let rows = db.select_all();

            for row in rows {
                println!("{:?}", row);
            }
        }
        Operation::Get => {
            let id = args.query.parse::<u32>().unwrap();

            let row = db.get_row(id);
            println!("{:?}", row);
        }
    }

    db.save(DB_PATH.to_string());

    Ok(())
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // create new dummy CSV file for testing purposes
        let contents = "id,first_name,last_name,email
0,Vilhelm,Prytz,vilhelm@prytznet.se";
        std::fs::write("test.csv", contents).unwrap();

        let mut db = Database::new("test.csv".to_string());

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

        // remove the dummy CSV file
        std::fs::remove_file("test.csv").unwrap();
    }
}

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

#[derive(Debug, PartialEq)]
enum Operation {
    Insert,
    Delete,
    SelectAll,
    Get,
    Interactive,
}

fn print_header(header: Vec<String>) {
    for column in header.iter() {
        print!("{: <20}", column);
    }
    println!();
}

fn print_row(row: Vec<String>) {
    for column in row.iter() {
        print!("{: <20}", column);
    }
    println!();
}

fn perform_operation(operation: Operation, query: String, db: &mut Database) {
    match operation {
        Operation::Insert => {
            // get the values from the query
            let values: Vec<&str> = query.split(",").collect();
            let values: Vec<String> = values.iter().map(|s| s.to_string()).collect();

            // insert the values into the database
            db.insert(values);

            println!("Inserted row");
        }
        Operation::Delete => {
            let id = query.parse::<u32>().unwrap();
            db.delete(id);
            println!("Deleted row with id {}", id)
        }
        Operation::SelectAll => {
            let rows = db.select_all();

            // print header, then all rows
            print_header(db.header.clone());

            for row in rows {
                print_row(row);
            }
        }
        Operation::Get => {
            let id = query.parse::<u32>().unwrap();

            let row = db.get_row(id);

            // print header and then row
            print_header(db.header.clone());
            print_row(row);
        }
        Operation::Interactive => panic!("This operation is not meant to be here!"),
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // map the operation to an enum
    let operation = match args.operation.as_str() {
        "insert" => Operation::Insert,
        "delete" => Operation::Delete,
        "select_all" => Operation::SelectAll,
        "get" => Operation::Get,
        "interactive" => Operation::Interactive,
        _ => panic!("Operation not supported"),
    };

    // create a new database
    let mut db = Database::new(DB_PATH.to_string());

    // if operation is interactive, start the interactive shell
    if operation == Operation::Interactive {
        loop {
            // get the operation from the user
            let mut operation = String::new();
            println!("Enter operation (insert, delete, select_all, get, quit):");
            std::io::stdin()
                .read_line(&mut operation)
                .expect("Failed to read line");
            let operation = operation.trim();

            match operation {
                "quit" => break,
                "select_all" => {
                    perform_operation(Operation::SelectAll, "".to_string(), &mut db);
                    continue;
                }
                _ => (),
            }

            // get the query from the user
            let mut query = String::new();
            println!("Enter query:");
            std::io::stdin()
                .read_line(&mut query)
                .expect("Failed to read line");
            let query = query.trim();

            // map the operation to an enum
            let operation = match operation {
                "insert" => Operation::Insert,
                "delete" => Operation::Delete,
                "select_all" => Operation::SelectAll,
                "get" => Operation::Get,
                "quit" => break,
                _ => panic!("Operation not supported"),
            };

            // execute the operation
            perform_operation(operation, query.to_string(), &mut db);
        }
    } else {
        // execute the operation
        perform_operation(operation, args.query, &mut db);
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

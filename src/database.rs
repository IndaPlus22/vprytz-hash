use crate::hash::HashMap;
use std::fs;

pub struct Database {
    pub map: HashMap,
}

impl Database {
    pub fn new(file_path: String) -> Database {
        let mut db = Database {
            map: HashMap::new(),
        };
        let contents = fs::read_to_string(file_path).expect("Unable to read file");
        let lines = contents.lines();
        for line in lines {
            let parts: Vec<&str> = line.split(",").collect();
            db.insert(parts[0].to_string(), parts[1].to_string());
        }
        db
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn delete(&mut self, key: String) {
        self.map.delete(key);
    }

    pub fn get(&self, key: String) {
        self.map.get(key);
    }

    pub fn select_all(&self) {
        println!("Selecting all");
    }
}

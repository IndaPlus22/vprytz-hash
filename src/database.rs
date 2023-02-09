use crate::hash::HashMap;
use std::fs;

pub struct Database {
    pub map: HashMap,
    pub next_id: u32,
    pub header: Vec<String>,
}

impl Database {
    pub fn new(file_path: String) -> Database {
        let mut db = Database {
            map: HashMap::new(),
            next_id: 0,
            header: vec![],
        };
        let contents = fs::read_to_string(file_path).expect("Unable to read file");
        let mut lines = contents.lines();

        // header with column names
        let header: Vec<&str> = lines.next().unwrap().split(",").collect();
        let header: Vec<String> = header.iter().map(|s| s.to_string()).collect();
        db.header = header.clone();

        for line in lines {
            let parts: Vec<&str> = line.split(",").collect();

            // id is always the first column
            let id = parts[0].parse::<u32>().unwrap();

            // iterate over each part of the row, and insert into the hashmap
            for (i, part) in parts.iter().enumerate() {
                let key = format!("{}-{}", id, header[i]);
                db.map.insert(key, part.to_string());
            }
            db.next_id = id + 1;
        }
        db
    }

    pub fn save(&self, file_path: String) {
        // get all the rows
        let mut rows = Vec::new();

        for i in 0..self.next_id {
            let row = self.get_row(i);
            rows.push(row);
        }

        // write the header
        let mut contents = self.header.join(",");
        contents.push_str("\n");

        // write the rows
        for row in rows.iter() {
            contents.push_str(&row.join(","));
            contents.push_str("\n");
        }

        // write the contents to the file
        fs::write(file_path, contents).expect("Unable to write file");
    }

    pub fn insert(&mut self, values: Vec<String>) {
        // insert each value into the hashmap
        self.map
            .insert(format!("{}-id", self.next_id), self.next_id.to_string());
        for (i, value) in values.iter().enumerate() {
            let key = format!("{}-{}", self.next_id, self.header[i + 1]);
            self.map.insert(key, value.to_string());
        }
        self.next_id += 1;
    }

    pub fn delete(&mut self, id: u32) {
        for column in self.header.iter() {
            let key = format!("{}-{}", id, column);
            self.map.delete(key);
        }
    }

    pub fn get_column(&self, id: u32, column: String) -> Option<String> {
        let key = format!("{}-{}", id, column);
        let elem = self.map.get(key);

        if elem.is_none() {
            return None;
        }

        return Some(elem.unwrap().value.clone());
    }

    pub fn get_row(&self, id: u32) -> Vec<String> {
        let mut row = Vec::new();

        for column in self.header.iter() {
            let key = format!("{}-{}", id, column);
            let elem = self.map.get(key);

            if elem.is_none() {
                continue;
            }

            row.push(elem.unwrap().value.clone());
        }
        return row;
    }

    pub fn select_all(&self) -> Vec<Vec<String>> {
        let mut all_columns = Vec::new();

        for i in 0..self.next_id {
            let mut row = Vec::new();
            for column in self.header.iter() {
                let key = format!("{}-{}", i, column);
                let elem = self.map.get(key);

                if elem.is_none() {
                    continue;
                }

                row.push(elem.unwrap().value.clone());
            }
            all_columns.push(row);
        }
        return all_columns;
    }
}

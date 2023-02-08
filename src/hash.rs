use std::collections::LinkedList;

#[derive(Clone)]
pub struct Element {
    pub key: String,
    pub value: String,
}

pub struct HashMap {
    data: Vec<Option<LinkedList<Element>>>,
}

impl HashMap {
    pub fn new() -> HashMap {
        // set length of data to 50
        const MAP_SIZE: usize = 100;
        HashMap {
            data: vec![None; MAP_SIZE],
        }
    }

    fn hash(&self, key: String) -> usize {
        // calculate integer sum of each character in key, then mod by length of data
        let mut sum = 0;
        for c in key.chars() {
            sum += c as usize;
        }
        sum % self.data.len()
    }

    pub fn insert(&mut self, key: String, value: String) {
        let index = self.hash(key.clone());
        let list = self.data[index].get_or_insert(LinkedList::new());

        list.push_back(Element { key, value });
    }

    pub fn delete(&mut self, key: String) {
        let index = self.hash(key.clone());

        self.data[index] = None;
    }

    pub fn get(&self, key: String) -> Option<Element> {
        let index = self.hash(key.clone());
        let list = self.data[index].clone();

        if list.is_none() {
            return None;
        }

        let list = list.unwrap();

        for element in list {
            if element.key == key {
                return Some(element);
            }
        }

        None
    }
}

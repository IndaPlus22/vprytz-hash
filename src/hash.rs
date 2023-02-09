#[derive(Clone, Debug)]
pub struct Element {
    pub key: String,
    pub value: String,
}

pub struct HashMap {
    data: Vec<Option<Vec<Element>>>,
    num_keys: usize,
}

impl HashMap {
    pub fn new() -> HashMap {
        // set length of data to 100
        const START_SIZE: usize = 100;
        HashMap {
            data: vec![None; START_SIZE],
            num_keys: 0,
        }
    }

    fn hash(&self, key: String, new_len: Option<usize>) -> usize {
        let length = match new_len {
            Some(len) => len,
            None => self.data.len(),
        };

        // calculate integer sum of each character in key, then mod by length of data
        let mut sum = 0;
        for c in key.chars() {
            sum += c as usize;
        }
        sum % length
    }

    pub fn insert(&mut self, key: String, value: String) {
        // check that we're not soon over 80% capacity
        if self.num_keys as f64 / self.data.len() as f64 > 0.8 {
            // double the size of the data
            let mut new_data = vec![None; self.data.len() * 2];

            // rehash all the elements
            for list in self.data.iter() {
                if list.is_none() {
                    continue;
                }

                for element in list.clone().unwrap().iter() {
                    let index = self.hash(element.key.clone(), Some(new_data.len()));
                    let new_list = new_data[index].get_or_insert(Vec::new());
                    new_list.push(element.clone());
                }
            }

            self.data = new_data;
        }

        let index = self.hash(key.clone(), None);
        let list = self.data[index].get_or_insert(Vec::new());

        list.push(Element { key, value });

        // increment key counter
        self.num_keys += 1;
    }

    pub fn delete(&mut self, key: String) {
        let index = self.hash(key.clone(), None);

        for (i, element) in self.data[index].clone().unwrap().iter().enumerate() {
            if element.key == key {
                self.data[index].as_mut().unwrap().remove(i);
            }
        }
    }

    pub fn get(&self, key: String) -> Option<Element> {
        let index = self.hash(key.clone(), None);
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

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = HashMap::new();

        map.insert("name".to_string(), "John".to_string());
        map.insert("age".to_string(), "20".to_string());

        assert_eq!(map.get("name".to_string()).unwrap().value, "John");
        assert_eq!(map.get("age".to_string()).unwrap().value, "20");

        map.delete("name".to_string());

        assert_eq!(map.get("name".to_string()).is_none(), true);
        assert_eq!(map.get("age".to_string()).unwrap().value, "20");
    }

    #[test]
    fn test_many_keys() {
        let mut map = HashMap::new();

        let keys = [
            "key1", "key2", "key3", "key4", "key5", "key6", "key7", "key8", "key9", "key10",
        ];
        let values = [
            "value1", "value2", "value3", "value4", "value5", "value6", "value7", "value8",
            "value9", "value10",
        ];

        for i in 0..10 {
            map.insert(keys[i].to_string(), values[i].to_string());
        }

        for i in 0..10 {
            assert_ne!(
                map.get(keys[i].to_string()).is_none(),
                true,
                "key: {}",
                keys[i]
            );
            assert_eq!(map.get(keys[i].to_string()).unwrap().value, values[i]);
        }
    }
}
